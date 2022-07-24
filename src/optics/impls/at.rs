use crate::{optics::affine_traversal::AsAffineTraversal, prelude::*};

pub struct At<Key>(Key);
impl<O> Optics<AsAffineTraversal, Vec<O>> for At<usize> {}
impl<O> Setter<AsAffineTraversal, Vec<O>> for At<usize> {
    type T = O;
    type O = O;

    type D = Vec<O>;

    fn set<F>(&self, source: Vec<O>, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> O,
    {
        source.into_iter().map(f).collect()
    }
}
impl<O> AffineFold<AsAffineTraversal, Vec<O>> for At<usize>
where
    O: Clone,
{
    type T = O;

    fn preview(&self, source: Vec<O>) -> Option<O> {
        source.get(self.0).cloned()
    }
}
impl<O> Fold<AsAffineTraversal, Vec<O>> for At<usize>
where
    O: Clone,
{
    type D = std::option::IntoIter<O>;

    fn fold(&self, source: Vec<O>) -> Self::D {
        source.get(self.0).cloned().into_iter()
    }
}
