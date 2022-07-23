use super::{And, Fold, Setter};

pub struct AsTraversal;
pub trait Traversal<As, S>
where
    Self: Fold<As, S>,
    Self::D: Iterator,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(<Self::D as Iterator>::Item) -> T;
}

impl<A1, A2, L1, L2, S> Traversal<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    <L1 as Fold<A1, S>>::D: Iterator,
    L2: Clone + Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    L2::D: Iterator,
    L1: Traversal<A1, S>,
    L2: Traversal<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(<Self::D as Iterator>::Item) -> T,
    {
        self.fold(source).map(f)
    }
}

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
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(S::Item) -> T,
    {
        source.into_iter().filter(self.0.clone()).map(f)
    }
}

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
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(S::Item) -> T,
    {
        source.into_iter().map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            lenses::{PersonMother, PersonName},
            Person,
        },
        optics::Then,
    };

    #[test]
    fn traversal() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Every.traverse(test, |x: String| x.to_uppercase());
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);

        let test: Vec<u32> = vec![1, 2, 3];

        let mut iter = Filtered(|x: &u32| x % 2 == 0).traverse(test, |x| x + 1);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_fold() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Every.fold(test);
        assert_eq!(iter.next().unwrap(), "foo");
        assert_eq!(iter.next().unwrap(), "bar");
        assert_eq!(iter.next(), None);

        let test: Vec<u32> = vec![1, 2, 3];

        let mut iter = Filtered(|x: &u32| x % 2 == 0).fold(test);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_setter() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let new: Vec<String> = Every.set(test, |x: String| x.to_uppercase());
        let mut iter = new.into_iter();
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);

        let test: Vec<u32> = vec![1, 2, 3];

        let new = Filtered(|x: &u32| x % 2 == 0).set(test, |x| x + 1);
        assert_eq!(new, vec![1, 3, 3]);
    }

    #[test]
    fn combinator() {
        let lens = PersonMother.then(PersonName);
        let mut iter = lens.traverse(Person::olivier(), |name| name.to_uppercase());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("ANNE".to_string()));
    }
}
