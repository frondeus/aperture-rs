use crate::{optics::traversal::AsTraversal, prelude::*};

#[derive(Clone)]
pub struct Filtered<Filter>(pub Filter);

impl<S, Filter> Fold<AsTraversal, S> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a S::Item) -> bool + Clone,
    S: IntoIterator,
{
    type D = std::iter::Filter<S::IntoIter, Filter>;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter().filter(self.0.clone())
    }
}

impl<S, Filter> Setter<AsTraversal, S> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a S::Item) -> bool + Clone,
    S: IntoIterator + FromIterator<S::Item>,
{
    type T = S::Item;
    type O = S::Item;

    type D = S;

    fn set<F>(&self, source: S, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> S::Item,
    {
        source
            .into_iter()
            .map(move |x| match (self.0.clone())(&x) {
                true => f(x),
                false => x,
            })
            .collect()
    }
}
impl<S, Filter> Traversal<AsTraversal, S> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a S::Item) -> bool + Clone,
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
        source.into_iter().filter(self.0.clone()).map(f)
    }
}
