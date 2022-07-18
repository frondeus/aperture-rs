use crate::identity::Identity;

use super::{AffineFold, Traversal};

pub struct AsAffineTraversal;
pub trait AffineTraversal<As, S, T, O, F>
where
    F: FnMut(T) -> O,
{
    fn map_opt(&self, source: S, f: F) -> Option<O>;
}

#[cfg(test)]
pub fn assert_affine_traversal<As, Optic, S, T>(_o: Optic)
where
    Optic: AffineTraversal<As, S, T, T, Identity>,
{
}

// impl<Optics, S, M, T, O, F> Traversal<AsAffineTraversal, S, T, O, F, M> for Optics
// where
//     Optics: AffineTraversal<M, S, T, O, F>,
//     F: FnMut(T) -> O,
// {
//     type Iter = std::option::IntoIter<O>;

//     fn map(&self, source: S, f: F) -> Self::Iter {
//         self.map_opt(source, f).into_iter()
//     }
// }

// impl<Optics, S, M, T> AffineFold<AsAffineTraversal, S, (M, T)> for Optics
// where
//     Optics: AffineTraversal<M, S, T, T, Identity>,
// {
//     type T = T;

//     fn preview(&self, source: S) -> Option<Self::T> {
//         self.map_opt(source, Identity)
//     }
// }

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{
        lazy::LazyExt,
        optics::{assert_traversal, fold::Fold},
    };

    use super::*;

    #[test]
    fn affine_traversal() {
        let test: Option<String> = Some("Foo".into());

        assert_affine_traversal(Option::<String>::as_ref);

        assert_eq!(
            Option::as_ref
                .map_opt(&test, |x| x.to_uppercase())
                .expect("some"),
            "FOO"
        );

        // let test: Option<String> = Some("Foo".into());
        // assert_eq!(Identity::option::<String>.fold(test).expect("some"), "Foo");
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
        // assert_traversal(Option::<String>::as_ref);

        // let test: Option<String> = Some("Foo".into());
        // assert_eq!(
        //     Option::as_ref
        //         .map(&test, |x| x.to_uppercase())
        //         .next()
        //         .expect("some"),
        //     "FOO"
        // );
    }
}
