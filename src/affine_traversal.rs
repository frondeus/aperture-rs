use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsAffineTraversal;
pub trait AffineTraversal<As, S>
// where
//     Self: AffineFold<As, S> + Traversal<As, S>,
//     <Self as Fold<As, S>>::D: Iterator,
{
    type O;
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(Self::O) -> T,
    {
        self.impl_preview(source).map(f)
    }

    fn impl_preview(&self, source: S) -> Option<Self::O>;

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O;
}

impl<X, S> AffineFold<AsAffineTraversal, S> for X
where
    X: AffineTraversal<AsAffineTraversal, S>,
{
    type T = X::O;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}

impl<As, X, S> Traversal<(AsAffineTraversal, As), S> for X
where
    X: AffineTraversal<As, S>,
    // S: IntoIterator + FromIterator<S::Item>,
{
    type D = std::option::IntoIter<X::O>;

    fn impl_fold(&self, source: S) -> Self::D {
        self.impl_preview(source).into_iter()
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(<Self::D as Iterator>::Item) -> <Self::D as Iterator>::Item,
    {
        // self.fold(source).map(f).into()
        // todo!()
        self.impl_set(source, f)
    }
}

// impl<X, S> Fold<AsAffineTraversal, S> for X
// where
//     X: AffineTraversal<AsAffineTraversal, S>,
//     S: IntoIterator + FromIterator<S::Item>,
// {
//     type D;

//     fn fold(&self, source: S) -> Self::D {
//         todo!()
//     }
// }
// impl

// impl<A1, A2, L1, L2, S, Item, SetT> AffineTraversal<(A1, A2), S> for And<L1, L2>
// where
//     L1: Fold<A1, S>,
//     <L1 as Fold<A1, S>>::D: Iterator,
//     L2: Clone + Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
//     <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,
//     <L2 as Fold<A2, <L1 as AffineFold<A1, S>>::T>>::D: Iterator<Item = Item>,
//     L1: Traversal<A1, S>,
//     L2: Traversal<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
//     L1: AffineFold<A1, S>,
//     L2: AffineFold<A2, <L1 as AffineFold<A1, S>>::T> + Clone,
//     L1: AffineTraversal<A1, S>,
//     L2: AffineTraversal<A2, <L1 as AffineFold<A1, S>>::T>,
//     Self: Fold<(A1, A2), S>,
//     <Self as Fold<(A1, A2), S>>::D: Iterator<Item = Item>,
//     L1: Setter<A1, S, T = SetT, O = SetT, D = S>,
//     L2: Setter<A2, SetT, D = SetT>,
// {
//     fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
//     where
//         F: FnOnce(<<Self as Fold<(A1, A2), S>>::D as Iterator>::Item) -> T,
//     {
//         self.0.preview(source).and_then(|t| self.1.map_opt(t, f))
//     }
// }
