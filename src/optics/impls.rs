use crate::method::Method;
use crate::optics::*;

use super::fold::FoldLike;

pub struct IsMethod;

impl<'a, S, M, T> ReviewLike<'a, S> for M
where
    M: Method<S, (), Output = T>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn review(&self, source: S) -> Self::T {
        self.mcall(source, ())
    }
}

impl<'a, S, M, T> SetLike<'a, S, IsMethod> for M
where
    M: Method<&'a mut S, (), Output = &'a mut T>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn set<F>(&self, source: &'a mut S, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        let _mut = self.mcall(source, ());
        f(_mut)
    }
}

impl<'a, T, M, I> FoldLike<'a, [T], IsMethod> for M
where
    M: Method<&'a [T], (), Output = I>,
    I: Iterator<Item = &'a T>,
    T: 'a,
{
    type T = T;

    type Iter = I;

    fn fold(&self, source: &'a [T]) -> Self::Iter {
        self.mcall(source, ())
    }
}

impl<'a, S, M, T> GetLike<'a, S, IsMethod> for M
where
    M: Method<&'a S, (), Output = &'a T>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn view(&self, source: &'a S) -> &'a Self::T {
        self.mcall(source, ())
    }
}

impl<'a, S, M, T> AffineFold<'a, S, IsMethod> for M
where
    M: Method<&'a S, (), Output = Option<&'a T>>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn preview(&self, source: &'a S) -> Option<&'a Self::T> {
        self.mcall(source, ())
    }
}
