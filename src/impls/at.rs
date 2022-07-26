use crate::prelude::*;

pub struct At<Key>(Key);
impl<O> AffineTraversal<AsAffineTraversal, Vec<O>> for At<usize> {
    type O = O;

    fn impl_preview(&self, source: Vec<O>) -> Option<Self::O> {
        source.into_iter().nth(self.0)
    }

    fn impl_set<F>(&self, source: Vec<O>, mut f: F) -> Vec<O>
    where
        F: FnMut(Self::O) -> Self::O,
    {
        source
            .into_iter()
            .enumerate()
            .map(|(i, o)| if i == self.0 { f(o) } else { o })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_affine_traversal() {
        let vec = vec![1, 2, 3];
        let first = At(1).map_opt(vec, |x| x + 5);
        assert_eq!(first, Some(7));
    }

    #[test]
    fn as_affine_fold() {
        let vec = vec![1, 2, 3];
        let first = At(1).preview(vec);
        assert_eq!(first, Some(2));
    }

    #[test]
    fn as_traversal() {
        let vec = vec![1, 2, 3];
        let mut iter = At(1).traverse(vec, |x| x + 5);
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_fold() {
        let vec = vec![1, 2, 3];
        let mut iter = At(1).fold(vec);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_setter() {
        let vec = vec![1, 2, 3];
        let new = At(1).set(vec, |x| x + 1);
        assert_eq!(new, vec!(1, 3, 3));
    }
}
