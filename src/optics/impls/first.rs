use crate::{optics::affine_traversal::AsAffineTraversal, prelude::*};

pub struct First;
impl<S> Fold<AsAffineTraversal, S> for First
where
    S: IntoIterator,
{
    type D = std::iter::Take<S::IntoIter>;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter().take(1)
    }
}
impl<S> AffineFold<AsAffineTraversal, S> for First
where
    S: IntoIterator,
{
    type T = S::Item;
    fn preview(&self, source: S) -> Option<<Self as AffineFold<AsAffineTraversal, S>>::T> {
        source.into_iter().next()
    }
}

impl<S> Setter<AsAffineTraversal, S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    type T = S::Item;
    type O = S::Item;

    type D = S;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> S::Item,
    {
        let mut iter = source.into_iter();
        let first = iter.next().map(f);
        first.into_iter().chain(iter).collect()
    }
}

impl<S> Traversal<AsAffineTraversal, S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    fn traverse<F, T>(
        &self,
        source: S,
        f: F,
    ) -> std::iter::Map<<Self as Fold<AsAffineTraversal, S>>::D, F>
    where
        F: FnMut(S::Item) -> T,
    {
        source.into_iter().take(1).map(f)
    }
}

impl<S> AffineTraversal<AsAffineTraversal, S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    // type O = S::Item;

    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(S::Item) -> T,
    {
        source.into_iter().next().map(f)
    }
}
