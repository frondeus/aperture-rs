use crate::identity::Identity;

use super::{fold::FoldLike, SetLike};

// Set + Fold

pub trait TraversalLike<S, T, O, F, Marker>
where
    F: FnMut(T) -> O,
{
    type Iter: Iterator<Item = O>;

    fn map(&self, source: S, f: F) -> Self::Iter;
}

pub struct IsTraversal;

impl<Optic, S, T, M> SetLike<S, (IsTraversal, M, T)> for Optic
where
    Optic: TraversalLike<S, T, T, Identity, M>,
{
    type T = S;

    fn set<F2>(&self, mut source: S, f: F2) -> S
    where
        F2: FnOnce(&mut S),
    {
        f(&mut source);
        source
    }
}

impl<Optic, S, T, M> FoldLike<S, (IsTraversal, M, T)> for Optic
where
    Optic: TraversalLike<S, T, T, Identity, M>,
{
    type T = T;

    type Iter = Optic::Iter;

    fn fold(&self, source: S) -> Self::Iter {
        self.map(source, Identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Traversal;
    impl<S, T, O, F> TraversalLike<S, T, O, F, Traversal> for Traversal
    where
        S: Iterator<Item = T>,
        F: FnMut(T) -> O,
    {
        type Iter = std::iter::Map<S, F>;

        fn map(&self, source: S, f: F) -> Self::Iter {
            source.map(f)
        }
    }

    #[test]
    fn traverse() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Traversal.map(test.into_iter(), |x: String| x.to_uppercase());

        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);

        let test: Vec<String> = vec!["foo".into(), "bar".into()];
        let mut iter = Vec::<String>::into_iter.map(test, |x: String| x.to_uppercase());
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);

        let test: Vec<String> = vec!["foo".into(), "bar".into()];
        let test: Vec<String> = Vec::<String>::into_iter.set(test, |x| *x = vec!["ABC".into()]);

        // Person::details::adress::street
    }
}
