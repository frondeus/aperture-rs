use super::{Fold, Setter};
use crate::method::Method;

pub mod nested;

pub struct AsTraversal;
pub trait Traversal<As, S, T, F>
where
    Self: Fold<As, S> + Setter<As, S, <Self as Traversal<As, S, T, F>>::O, D = S>,
    F: FnMut(<Self as Traversal<As, S, T, F>>::O) -> T,
{
    type O;
    type D: Iterator<Item = T>;

    fn traverse(&self, source: S, f: F) -> <Self as Traversal<As, S, T, F>>::D;
}

struct Filtered<Filter>(Filter);

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

impl<S, Filter> Setter<AsTraversal, S, S::Item> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a S::Item) -> bool + Clone,
    S: IntoIterator + FromIterator<S::Item>,
{
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

impl<S, F, T, Filter> Traversal<AsTraversal, S, T, F> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a S::Item) -> bool + Clone,
    S: IntoIterator + FromIterator<S::Item>,
    F: FnMut(S::Item) -> T,
{
    type O = S::Item;

    type D = std::iter::Map<std::iter::Filter<S::IntoIter, Filter>, F>;

    fn traverse(&self, source: S, f: F) -> <Self as Traversal<AsTraversal, S, T, F>>::D {
        source.into_iter().filter(self.0.clone()).map(f)
    }
}

struct Every;

impl<S> Fold<AsTraversal, S> for Every
where
    S: IntoIterator,
{
    type D = S::IntoIter;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter()
    }
}

impl<S, T> Setter<AsTraversal, S, T> for Every
where
    S: IntoIterator<Item = T> + FromIterator<T>,
    // for<'a> &'a mut S: IntoIterator<Item = &'a mut T>,
{
    type O = T;

    type D = S;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(T) -> T,
    {
        // let _mut: &mut S = &mut source;
        // _mut.into_iter().for_each(|x| {
        //     *x = f(*x);
        // });
        // .map(|x| {
        //     *x = f;
        //     x
        // });
        // source
        source.into_iter().map(f).collect()
        // todo!()
    }
}

impl<S, F, T> Traversal<AsTraversal, S, T, F> for Every
where
    S: IntoIterator + FromIterator<S::Item>,
    // for<'a> &'a mut S: IntoIterator<Item = S::Item>,
    F: FnMut(S::Item) -> T,
{
    type O = S::Item;

    type D = std::iter::Map<S::IntoIter, F>;

    fn traverse(&self, source: S, f: F) -> <Self as Traversal<AsTraversal, S, T, F>>::D {
        source.into_iter().map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
