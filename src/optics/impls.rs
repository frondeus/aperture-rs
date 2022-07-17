use crate::method::{Method, MethodOnce};
use crate::optics::*;

pub struct IsMethod;

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

impl<'a, S, M, T> GetLikeOnce<'a, S> for M
where
    M: MethodOnce<&'a S, (), Output = &'a T>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn view_once(self, source: &'a S) -> &'a Self::T {
        self.mcall_once(source, ())
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

impl<'a, S, M, T> SetLikeOnce<'a, S> for M
where
    M: MethodOnce<&'a mut S, (), Output = &'a mut T>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn set_once<F>(self, source: &'a mut S, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        let _mut = self.mcall_once(source, ());
        f(_mut)
    }
}

impl<'a, S, M, T> TraversalLike<'a, S, IsMethod> for M
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
impl<'a, S, M, T> ReviewLikeOnce<'a, S> for M
where
    M: MethodOnce<S, (), Output = T>,
    T: 'a,
    S: 'a,
{
    type T = T;

    fn review_once(self, source: S) -> Self::T {
        self.mcall_once(source, ())
    }
}
