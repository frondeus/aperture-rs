use std::marker::PhantomData;

use super::{Traversal, TraversalRef};

pub struct NestedTraversal<As, I, T>
where
    I: Iterator,
    T: Traversal<I::Item, As>,
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
    T: Traversal<I::Item, As>,
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
    T: Traversal<I::Item, As>,
    T::D: Iterator,
{
    type Item = <<T as Traversal<I::Item, As>>::D as Iterator>::Item;

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
    OUT: TraversalRef<S, AsO>,
    INN: TraversalRef<<OUT::D as Iterator>::Item, AsI>,
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
    OUT: TraversalRef<S, AsO>,
    INN: TraversalRef<<OUT::D as Iterator>::Item, AsI>,
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
    OUT: TraversalRef<S, AsO>,
    INN: TraversalRef<<OUT::D as Iterator>::Item, AsI>,
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
