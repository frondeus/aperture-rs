use std::marker::PhantomData;

use super::{Traversal, TraversalRef};

pub struct NestedTraversal<As, I, T>
where
    I: Iterator,
    T: Traversal<As, I::Item>,
    T::D: Iterator,
{
    outer: I,
    inner: T,
    _as: PhantomData<As>,
    last: Option<T::D>,
}
impl<As, I, T> NestedTraversal<As, I, T>
where
    I: Iterator,
    T: Traversal<As, I::Item>,
    T::D: Iterator,
{
    pub fn new(i: I, f: T) -> Self {
        Self {
            inner: f,
            outer: i,
            _as: PhantomData,
            last: None,
        }
    }
}
impl<As, I, T> Iterator for NestedTraversal<As, I, T>
where
    I: Iterator,
    T: Traversal<As, I::Item>,
    T::D: Iterator,
{
    type Item = <<T as Traversal<As, I::Item>>::D as Iterator>::Item;

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
                    self.last = Some(self.inner.impl_fold(inner));
                }
            }
        }
    }
}

pub struct NestedTraversalRef<'a, AsO, AsI, OUT, INN, S>
where
    OUT: TraversalRef<AsO, S>,
    INN: TraversalRef<AsI, <OUT::D as Iterator>::Item>,
    S: 'a,
    <OUT::D as Iterator>::Item: 'a,
    <INN::D as Iterator>::Item: 'a,
{
    outer: OUT::DRef<'a>,
    inner: INN,
    _as: PhantomData<&'a (AsI, AsO)>,
    last: Option<INN::DRef<'a>>,
}

impl<'a, AsO, AsI, OUT, INN, S> NestedTraversalRef<'a, AsO, AsI, OUT, INN, S>
where
    OUT: TraversalRef<AsO, S>,
    INN: TraversalRef<AsI, <OUT::D as Iterator>::Item>,
    S: 'a,
    <OUT::D as Iterator>::Item: 'a,
    <INN::D as Iterator>::Item: 'a,
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

impl<'a, AsO, AsI, OUT, INN, S> Iterator for NestedTraversalRef<'a, AsO, AsI, OUT, INN, S>
where
    OUT: TraversalRef<AsO, S>,
    INN: TraversalRef<AsI, <OUT::D as Iterator>::Item>,
    S: 'a,
    <OUT::D as Iterator>::Item: 'a,
    <INN::D as Iterator>::Item: 'a,
{
    type Item = &'a <INN::D as Iterator>::Item;

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
                Some(inner) => self.last = Some(self.inner.impl_fold_ref(inner)),
            }
        }
    }
}
