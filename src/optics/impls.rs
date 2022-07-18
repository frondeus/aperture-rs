use super::fold::{AsFold, Fold};
use crate::{
    method::{Method, MethodOnce},
    optics::*,
};

// pub struct IsMethod;

impl<S, M, T> Setter<AsSetter, S> for M
where
    M: for<'a> Method<&'a mut S, (), Output = &'a mut T>,
{
    type T = T;

    fn set<F>(&self, mut source: S, f: F) -> S
    where
        F: FnOnce(&mut T),
    {
        let _mut = self.mcall(&mut source, ());
        f(_mut);
        source
    }
}

// Fold is implemented automatically by Traversal
impl<T, M, I, S> Fold<AsFold, S> for M
where
    M: Method<S, (), Output = I>,
    I: Iterator<Item = T>,
{
    type T = T;

    type Iter = I;

    fn fold(&self, source: S) -> Self::Iter {
        self.mcall(source, ())
    }
}

impl<S, M, T> AffineFold<AsAffineFold, S> for M
where
    M: Method<S, (), Output = Option<T>>,
{
    type T = T;
    fn preview(&self, source: S) -> Option<Self::T> {
        self.mcall(source, ())
    }
}

impl<S, M, SI, T> Traversal<AsTraversal, S> for M
where
    M: Method<S, (), Output = SI>,
    SI: Iterator<Item = T>,
{
    type Iter = std::iter::Map<SI, Self::F>;

    fn map(&self, source: S, f: Self::F) -> Self::Iter {
        let si = self.mcall(source, ()).into_iter();
        si.map(f)
    }
}

impl<S, M, T> Review<AsReview, S> for M
where
    M: Method<S, (), Output = T>,
{
    type T = T;

    fn review(&self, source: S) -> Self::T {
        self.mcall(source, ())
    }
}

impl<S, M, T, O, F> AffineTraversal<AsAffineTraversal, S, T, O, F> for M
where
    M: Method<S, (), Output = Option<T>>,
    F: FnMut(T) -> O,
{
    fn map_opt(&self, source: S, f: F) -> Option<O> {
        self.mcall(source, ()).map(f)
    }
}

// impl<'a, S, M, T> GetLike<'a, S, IsMethod> for M
// where
//     M: Method<&'a S, (), Output = &'a T>,
//     T: 'a,
//     S: 'a,
// {
//     type T = T;

//     fn view(&self, source: &'a S) -> &'a Self::T {
//         self.mcall(source, ())
//     }
// }
