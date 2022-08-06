use std::marker::PhantomData;

use super::{Fold, FoldRef};

pub struct NestedFold<As, I, F>
where
    I: Iterator,
    F: Fold<As, I::Item>,
    F::D: Iterator,
{
    outer: I,
    inner: F,
    _as: PhantomData<As>,
    last: Option<F::D>,
}
impl<As, I, F> NestedFold<As, I, F>
where
    I: Iterator,
    F: Fold<As, I::Item>,
    F::D: Iterator,
{
    pub fn new(i: I, f: F) -> Self {
        Self {
            inner: f,
            outer: i,
            _as: PhantomData,
            last: None,
        }
    }
}
impl<As, I, F> Iterator for NestedFold<As, I, F>
where
    I: Iterator,
    F: Fold<As, I::Item>,
    F::D: Iterator,
{
    type Item = <<F as Fold<As, I::Item>>::D as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.last {
                match inner.next() {
                    elt @ Some(_) => return elt,
                    None => self.last = None,
                }
            }
            match self.outer.next() {
                None => return None,
                Some(inner) => {
                    self.last = Some(self.inner.fold(inner));
                }
            }
        }
    }
}

pub struct NestedFoldRef<'a, AsO, AsI, OUT, INN, S>
where
    OUT: FoldRef<AsO, S>,
    OUT::D: Iterator,
    INN: FoldRef<AsI, OUT::Item<'a>>,
    INN::D: Iterator,
    S: 'a,
{
    outer: OUT::DRef<'a>,
    inner: INN,
    _as: PhantomData<&'a (AsI, AsO)>,
    last: Option<INN::DRef<'a>>,
}

impl<'a, AsO, AsI, OUT, INN, S> NestedFoldRef<'a, AsO, AsI, OUT, INN, S>
where
    OUT: FoldRef<AsO, S>,
    OUT::D: Iterator,
    INN: FoldRef<AsI, OUT::Item<'a>>,
    INN::D: Iterator,
    S: 'a,
{
    pub fn new(outer: OUT::DRef<'a>, inner: INN) -> Self {
        Self {
            outer,
            inner,
            _as: PhantomData,
            last: None,
        }
    }
}

impl<'a, AsO, AsI, OUT, INN, S> Iterator for NestedFoldRef<'a, AsO, AsI, OUT, INN, S>
where
    OUT: FoldRef<AsO, S>,
    OUT::D: Iterator,
    INN: FoldRef<AsI, OUT::Item<'a>>,
    INN::D: Iterator,
    S: 'a,
{
    type Item = &'a INN::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.last {
                match inner.next() {
                    elt @ Some(_) => return elt,
                    None => self.last = None,
                }
            }
            match self.outer.next() {
                None => return None,
                Some(inner) => self.last = Some(self.inner.fold_ref(inner)),
            }
        }
    }
}
