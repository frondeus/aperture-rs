use super::{ReviewLike, SetLike, TraversalLike};

pub trait PrismLike<'a, S, TM>: ReviewLike<'a, S> + TraversalLike<'a, S, TM> {}

impl<'a, S, TM, P> PrismLike<'a, S, TM> for P where P: ReviewLike<'a, S> + TraversalLike<'a, S, TM> {}

pub struct At<T>(pub T);

impl<'a, S> ReviewLike<'a, Vec<S>> for At<usize>
where
    S: 'a,
{
    type T = S;

    fn review(&self, mut source: Vec<S>) -> Self::T {
        source.swap_remove(self.0)
    }
}

impl<'a, S, TM> TraversalLike<'a, Vec<S>, TM> for At<usize>
where
    S: 'a,
{
    type T = S;

    fn preview(&self, source: &'a Vec<S>) -> Option<&'a Self::T> {
        source.get(self.0)
    }
}

impl<'a, S, SM> SetLike<'a, Vec<S>, SM> for At<usize>
where
    S: 'a,
{
    type T = S;

    fn set<F>(&self, source: &'a mut Vec<S>, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        if let Some(t) = source.get_mut(self.0) {
            f(t)
        }
    }
}

pub trait PrismVecExt<T> {
    fn at(index: usize) -> At<usize>;
}

impl<T> PrismVecExt<T> for Vec<T> {
    fn at(index: usize) -> At<usize> {
        At(index)
    }
}
