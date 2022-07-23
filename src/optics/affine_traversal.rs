use crate::prelude::*;

pub struct AsAffineTraversal;
pub trait AffineTraversal<As, S>
where
    Self: AffineFold<As, S> + Traversal<As, S>,
    <Self as Fold<As, S>>::D: Iterator,
{
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(<<Self as Fold<As, S>>::D as Iterator>::Item) -> T;
}

impl<A1, A2, L1, L2, S, Item, SetT> AffineTraversal<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    <L1 as Fold<A1, S>>::D: Iterator,
    L2: Clone + Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,
    <L2 as Fold<A2, <L1 as AffineFold<A1, S>>::T>>::D: Iterator<Item = Item>,
    L1: Traversal<A1, S>,
    L2: Traversal<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    L1: AffineFold<A1, S>,
    L2: AffineFold<A2, <L1 as AffineFold<A1, S>>::T> + Clone,
    L1: AffineTraversal<A1, S>,
    L2: AffineTraversal<A2, <L1 as AffineFold<A1, S>>::T>,
    Self: Fold<(A1, A2), S>,
    <Self as Fold<(A1, A2), S>>::D: Iterator<Item = Item>,
    L1: Setter<A1, S, T = SetT, O = SetT, D = S>,
    L2: Setter<A2, SetT, D = SetT>,
{
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(<<Self as Fold<(A1, A2), S>>::D as Iterator>::Item) -> T,
    {
        self.0.preview(source).and_then(|t| self.1.map_opt(t, f))
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
        prelude::{first::First, Then},
    };

    #[test]
    fn as_traversal() {
        let vec = vec![1, 2, 3];
        let mut iter = First.traverse(vec, |x| x + 1);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_affine_fold() {
        let vec = vec![1, 2, 3];
        assert_eq!(First.preview(vec), Some(1));
    }

    #[test]
    fn as_setter() {
        let vec = vec![1, 2, 3];
        let new = First.set(vec, |x| x + 1);
        assert_eq!(new, vec!(2, 2, 3));
    }

    #[test]
    fn as_fold() {
        let vec = vec![1, 2, 3];
        let mut iter = First.fold(vec);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_aff_traversal() {
        let lens = PersonMother.then(PersonName);
        let mums_name = lens.map_opt(Person::olivier(), |name| name.to_uppercase());
        assert_eq!(mums_name, Some("ANNE".to_string()));
    }
}
