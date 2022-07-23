use super::{AffineFold, Fold, Setter, Traversal};
use crate::method::Method;

pub struct AsAffineTraversal;
pub trait AffineTraversal<As, S>
where
    Self: AffineFold<As, S> + Traversal<As, S>,
    Self::D: Iterator,
{
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(<Self::D as Iterator>::Item) -> T;
}

// #[cfg(test)]
// pub fn assert_affine_traversal<As, Optic, S, Out>(_o: Optic)
// where
//     Optic: AffineTraversal<As, S, Out, crate::identity::Identity, In = Out>,
// {
// }
pub struct First;
impl<S> Fold<AsAffineTraversal, S> for First
where
    S: IntoIterator,
{
    type D = std::iter::Take<S::IntoIter>;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter().take(1)
    }
}
impl<S> AffineFold<AsAffineTraversal, S> for First
where
    S: IntoIterator,
{
    type T = S::Item;
    fn preview(&self, source: S) -> Option<<Self as AffineFold<AsAffineTraversal, S>>::T> {
        source.into_iter().next()
    }
}

impl<S> Setter<AsAffineTraversal, S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    type T = S::Item;
    type O = S::Item;

    type D = S;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> S::Item,
    {
        let mut iter = source.into_iter();
        let first = iter.next().map(f);
        first.into_iter().chain(iter).collect()
    }
}

impl<S> Traversal<AsAffineTraversal, S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(S::Item) -> T,
    {
        source.into_iter().take(1).map(f)
    }
}

impl<S> AffineTraversal<AsAffineTraversal, S> for First
where
    S: IntoIterator + FromIterator<S::Item>,
{
    // type O = S::Item;

    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(S::Item) -> T,
    {
        source.into_iter().next().map(f)
    }
}

pub struct At<Key>(Key);
// impl<O, T, F> AffineTraversal<AsAffineTraversal, Vec<O>, T, F> for At<usize> {
//     fn map_opt(&self, source: Vec<O>, f: F) -> Option<T> {
//         source.get(self.A).cloned()
//     }
// }
// impl<O, T, F> Traversal<AsAffineTraversal, Vec<O>, T, F> for At<usize> {
//     type O;

//     type D;

//     fn traverse(
//         &self,
//         source: Vec<O>,
//         f: F,
//     ) -> <Self as Traversal<AsAffineTraversal, Vec<O>, T, F>>::D {
//         todo!()
//     }
// }
impl<O> Setter<AsAffineTraversal, Vec<O>> for At<usize> {
    type T = O;
    type O = O;

    type D = Vec<O>;

    fn set<F>(&self, source: Vec<O>, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> O,
    {
        source.into_iter().collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity;

    #[test]
    fn affine_traversal() {
        // let test: Option<String> = Some("Foo".into());

        // assert_eq!(
        //     Option::as_ref
        //         .map_opt(&test, |x| x.to_uppercase())
        //         .expect("some"),
        //     "FOO"
        // );

        // let test: Option<String> = Some("Foo".into());
        // assert_eq!(
        //     Identity::option::<String>
        //         .map_opt(test, |x| x.to_uppercase())
        //         .expect("some"),
        //     "FOO"
        // );
        // let mut map: HashMap<usize, String> = HashMap::new();
        // map.insert(1, "Foo".into());

        // let test: Option<String> = Some("Foo".into());
        // assert_eq!(
        //     HashMap::get.with_args((&1,)).preview(&map).expect("some"),
        //     "Foo"
        // );
    }

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
}
