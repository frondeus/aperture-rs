use super::{Fold, Setter};
use crate::method::Method;

pub struct AsTraversal;
pub trait Traversal<As, S, Out, F>
where
    Self: Setter<As, S, In = <Self as Fold<As, S>>::T> + Fold<As, S>,
    F: FnMut(<Self as Fold<As, S>>::T) -> Out,
{
    type TraversalIter: Iterator<Item = Out>;

    fn map(&self, source: S, f: F) -> Self::TraversalIter;
}

// #[cfg(test)]
// pub fn assert_traversal<As, Optic, S, T>(_o: Optic)
// where
//     Optic: Traversal<As, S, T, T>,
// {
// }
impl<S, M, SI, In, F, Out> Traversal<AsTraversal, S, Out, F> for M
where
    M: Method<S, (), Output = SI> + Setter<AsTraversal, S, In = In> + Fold<AsTraversal, S, T = In>,
    SI: Iterator<Item = In>,
    F: FnMut(In) -> Out,
{
    type TraversalIter = std::iter::Map<SI, F>;

    fn map(&self, source: S, f: F) -> Self::TraversalIter
where {
        let si = self.mcall(source, ()).into_iter();
        si.map(f)
    }
}

impl<S, M, SI, In> Setter<AsTraversal, S> for M
where
    M: Method<S, (), Output = SI>,
    SI: Iterator<Item = In>,
{
    type In = In;

    fn set<F>(&self, mut source: S, f: F) -> S
    where
        F: FnOnce(&mut Self::In),
    {
        // f(&mut source);
        // source
        todo!()
    }
}

impl<S, M, SI, In> Fold<AsTraversal, S> for M
where
    M: Method<S, (), Output = SI>,
    SI: Iterator<Item = In>,
{
    type T = In;

    type FoldIter = SI;

    fn fold(&self, source: S) -> Self::FoldIter {
        self.mcall(source, ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traversal() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Vec::<String>::into_iter.map(test, |x: String| x.to_uppercase());
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_set() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        // assert_setter(Vec::<String>::into_iter);

        let test: Vec<String> = Vec::<String>::into_iter.set(test, |x| *x = "ABC".into());
        // let mut iter = test.into_iter();
        // assert_eq!(iter.next().unwrap(), "ABC");
        // assert_eq!(iter.next(), None);
    }
}
