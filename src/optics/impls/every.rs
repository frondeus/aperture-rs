use crate::{optics::traversal::AsTraversal, prelude::*};

#[derive(Clone)]
pub struct Every;

impl<S> Fold<AsTraversal, S> for Every
where
    S: IntoIterator,
{
    type D = S::IntoIter;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter()
    }
}

impl<S, T> Setter<AsTraversal, S> for Every
where
    S: IntoIterator<Item = T> + FromIterator<T>,
    // for<'a> &'a mut S: IntoIterator<Item = &'a mut T>,
{
    type T = T;
    type O = T;

    type D = S;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(T) -> T,
    {
        source.into_iter().map(f).collect()
    }
}

impl<S> Traversal<AsTraversal, S> for Every
where
    S: IntoIterator + FromIterator<S::Item>,
{
    fn traverse<F, T>(
        &self,
        source: S,
        f: F,
    ) -> std::iter::Map<<Self as Fold<AsTraversal, S>>::D, F>
    where
        F: FnMut(S::Item) -> T,
    {
        source.into_iter().map(f)
    }
}
