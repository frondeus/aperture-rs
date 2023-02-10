use crate::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct First;
impl<S> AffineTraversal<S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    type O = S::Item;

    fn impl_preview(&self, source: S) -> Option<Self::O> {
        source.into_iter().next()
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: FnMut(Self::O) -> Self::O,
    {
        let mut iter = source.into_iter();
        let first = iter.next().map(f);
        first.into_iter().chain(iter).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_affine_traversal() {
        let vec = vec![1, 2, 3];
        let first = First.map_opt(vec, |x| x + 5);
        assert_eq!(first, Some(6));
    }

    #[test]
    fn as_affine_fold() {
        let vec = vec![1, 2, 3];
        let first = First.preview(vec);
        assert_eq!(first, Some(1));
    }

    #[test]
    fn as_traversal() {
        let vec = vec![1, 2, 3];
        let mut iter = First.traverse(vec, |x| x + 5);
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_fold() {
        let vec = vec![1, 2, 3];
        let mut iter = First.fold(vec);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_setter() {
        let vec = vec![1, 2, 3];
        let new = First.set(vec, |x| x + 1);
        assert_eq!(new, vec!(2, 2, 3));
    }
}
