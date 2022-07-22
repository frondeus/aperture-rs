use super::Fold;
use crate::method::Method;

pub struct AsAffineFoldMethod;
pub trait AffineFold<As, S>: Fold<As, S> {
    type T;
    fn preview(&self, source: S) -> Option<<Self as AffineFold<As, S>>::T>;
}

// Since aff fold is basically aff traversal with identity function it is automatically implemented
// impl<S, M, T> AffineFold<AsAffineFoldMethod, S> for M
// where
//     M: Method<S, (), Output = Option<T>> + Fold<AsAffineFoldMethod, S, T = T>,
// {
//     fn preview(&self, source: S) -> Option<Self::T> {
//         self.mcall(source, ())
//     }
// }

// impl<S, M, T> Fold<AsAffineFoldMethod, S> for M
// where
//     M: Method<S, (), Output = Option<T>>,
// {
//     type T = T;
//     type FoldIter = std::option::IntoIter<T>;
//     fn fold(&self, source: S) -> Self::FoldIter {
//         self.mcall(source, ()).into_iter()
//     }
// }
// #[cfg(test)]
// pub fn assert_affine_fold<As, Optic, S>(_o: Optic)
// where
//     Optic: AffineFold<As, S>,
// {
// }

// Affine traversal took it
// #[cfg(test)]
// mod tests {

//     use std::collections::HashMap;

//     use super::*;
//     use crate::{data::Test, lazy::LazyExt};

//     #[test]
//     fn affine_fold() {
//         let vec = vec![1, 2, 3];
//         assert_eq!(First.preview(vec), Some(1));

//         let vec = vec![1, 2, 3];
//         let mut iter = First.fold(vec);
//         assert_eq!(iter.next(), Some(1));
//         assert_eq!(iter.next(), None);
//         // let test = Test("Foo".into());
//         // assert_eq!(Test::own_opt.preview(test).expect("some"), "Foo");
//         // let test: Option<String> = Some("Foo".into());
//         // assert_eq!(Option::as_ref.preview(&test).expect("some"), "Foo");

//         // let mut map: HashMap<usize, String> = HashMap::new();
//         // map.insert(1, "Foo".into());

//         // assert_eq!(
//         //     HashMap::get.with_args((&1,)).preview(&map).expect("some"),
//         //     "Foo"
//         // );
//         // assert_eq!(HashMap::get.with_args((&2,)).preview(&map), None);

//         // assert_eq!(Option::or.fold(test).next().expect("some"), "Foo");
//     }

//     #[test]
//     fn as_fold() {
//         let test: Option<String> = Some("Foo".into());

//         // assert_fold(Option::<String>::as_ref);
//         // assert_affine_fold(Option::<String>::as_ref);

//         // let mut iter = Option::as_ref.fold(&test);
//         // assert_eq!(iter.next().expect("some"), "Foo");
//     }
// }
