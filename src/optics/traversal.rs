use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsTraversal;
pub trait Traversal<As, S>
where
    Self: Fold<As, S> + Setter<As, S>,
    <Self as Fold<As, S>>::D: Iterator,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<<Self as Fold<As, S>>::D, F>
    where
        F: FnMut(<<Self as Fold<As, S>>::D as Iterator>::Item) -> T;
}

impl<A1, A2, L1, L2, S, SetT> Traversal<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    <L1 as Fold<A1, S>>::D: Iterator,
    L2: Clone + Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,
    L1: Traversal<A1, S>,
    L2: Traversal<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    L1: Setter<A1, S, T = SetT, O = SetT, D = S>,
    L2: Setter<A2, SetT, D = SetT>,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<<Self as Fold<(A1, A2), S>>::D, F>
    where
        F: FnMut(<<Self as Fold<(A1, A2), S>>::D as Iterator>::Item) -> T,
    {
        self.fold(source).map(f)
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
        prelude::{every::Every, filtered::Filtered},
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
