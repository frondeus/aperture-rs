use std::marker::PhantomData;

use super::Fold;

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
