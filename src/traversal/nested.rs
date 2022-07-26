use std::marker::PhantomData;

use super::Traversal;
use crate::prelude::AffineFold;

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
pub struct NestedAF<As, I, AF>
where
    I: Iterator,
    AF: AffineFold<As, I::Item>,
{
    outer: I,
    inner: AF,
    _as: PhantomData<As>,
}
impl<As, I, AF> NestedAF<As, I, AF>
where
    I: Iterator,
    AF: AffineFold<As, I::Item>,
{
    pub fn new(i: I, f: AF) -> Self {
        Self {
            inner: f,
            outer: i,
            _as: PhantomData,
        }
    }
}
impl<As, I, AF> Iterator for NestedAF<As, I, AF>
where
    I: Iterator,
    AF: AffineFold<As, I::Item>,
{
    type Item = AF::T;

    fn next(&mut self) -> Option<Self::Item> {
        // loop {
        //     if let Some(ref mut inner) = self.last {
        //         match inner.next() {
        //             elt @ Some(_) => return elt,
        //             None => self.last = None,
        //         }
        //     }
        //     match self.outer.next() {
        //         None => return None,
        //         Some(inner) => {
        //             self.last = Some(self.inner.impl_fold(inner));
        //         }
        //     }
        // }
        let item = self.outer.next()?;
        self.inner.preview(item)
    }
}