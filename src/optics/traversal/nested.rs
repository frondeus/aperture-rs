use std::marker::PhantomData;

use super::Fold;

pub struct NestedTraverse<As, I, T, F>
where
    I: Iterator,
    T: Fold<As, I::Item>,
    T::D: Iterator,
{
    outer: I,
    inner: T,
    func: F,
    _as: PhantomData<As>,
    last: Option<T::D>,
}
impl<As, I, T, F> NestedTraverse<As, I, T, F>
where
    I: Iterator,
    T: Fold<As, I::Item>,
    T::D: Iterator,
{
    pub fn new(i: I, f: T, func: F) -> Self {
        Self {
            inner: f,
            outer: i,
            func,
            _as: PhantomData,
            last: None,
        }
    }
}
impl<As, I, T, F> Iterator for NestedTraverse<As, I, T, F>
where
    I: Iterator,
    T: Fold<As, I::Item>,
    T::D: Iterator,
{
    type Item = <<T as Fold<As, I::Item>>::D as Iterator>::Item;

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
