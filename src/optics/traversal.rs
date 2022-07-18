use crate::identity::Identity;

use super::{fold::Fold, Setter};

// Set + Fold

pub struct AsTraversal;
pub trait Traversal<As, S> {
    type T;
    type O;
    type F: FnMut(Self::T) -> Self::O;
    type Iter: Iterator<Item = Self::O>;

    fn map(&self, source: S, f: Self::F) -> Self::Iter;
}

impl<Optic, As, S> Setter<(AsTraversal, As), S> for Optic
where
    Optic: Traversal<As, S>,
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

// impl<Optic, S, T, M> Fold<AsTraversal, S, (M, T)> for Optic
// where
//     Optic: Traversal<M, S, T, T, Identity, Optic>,
// {
//     type T = T;

//     type Iter = Optic::Iter;

//     fn fold(&self, source: S) -> Self::Iter {
//         self.map(source, Identity)
//     }
// }

#[cfg(test)]
pub fn assert_traversal<As, Optic, S, M>(_o: Optic)
where
    // Optic: Traversal<As, S, T, T, Identity, M>,
    Optic: Traversal<As, S>,
{
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::optics::assert_setter;

    use super::*;

    struct AsTraverse;
    struct Traverse<Mark>(PhantomData<Mark>);
    impl<S, T, O, F> Traversal<AsTraverse, S> for Traverse<(O, F)>
    where
        S: Iterator<Item = T>,
        F: FnMut(T) -> O,
    {
        type Iter = std::iter::Map<S, F>;
        type T = T;

        type O = O;

        type F = F;

        fn map(&self, source: S, f: F) -> Self::Iter {
            source.map(f)
        }
    }

    #[test]
    fn traversal() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Traverse(PhantomData).map(test.into_iter(), |x: String| x.to_uppercase());

        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);

        let test: Vec<String> = vec!["foo".into(), "bar".into()];
        let mut iter = Vec::<String>::into_iter.map(test, |x: String| x.to_uppercase());
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_set() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        assert_setter(Vec::<String>::into_iter);

        let test: Vec<String> = Vec::<String>::into_iter.set(test, |x| *x = vec!["ABC".into()]);
    }
}
