use super::fold::Fold;

pub struct AsAffineFold;
pub trait AffineFold<As, S> {
    type T;
    fn preview(&self, source: S) -> Option<Self::T>;
}

#[cfg(test)]
pub fn assert_affine_fold<As, Optic, S>(_o: Optic)
where
    Optic: AffineFold<As, S>,
{
}

// impl<T, S, M, AF, Optics> Fold<AsAffineFold, S> for Optics
// where
//     Optics: AffineFold<AF, S, T = T>,
// {
//     type T = T;

//     type Iter = std::option::IntoIter<T>;

//     fn fold(&self, source: S) -> Self::Iter {
//         self.preview(source).into_iter()
//     }
// }

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;
    use crate::{data::Test, lazy::LazyExt, optics::fold::assert_fold};

    #[test]
    fn affine_fold() {
        let test = Test("Foo".into());
        assert_eq!(Test::own_opt.preview(test).expect("some"), "Foo");
        let test: Option<String> = Some("Foo".into());
        assert_eq!(Option::as_ref.preview(&test).expect("some"), "Foo");

        let mut map: HashMap<usize, String> = HashMap::new();
        map.insert(1, "Foo".into());

        assert_eq!(
            HashMap::get.with_args((&1,)).preview(&map).expect("some"),
            "Foo"
        );
        assert_eq!(HashMap::get.with_args((&2,)).preview(&map), None);

        // assert_eq!(Option::or.fold(test).next().expect("some"), "Foo");
    }

    #[test]
    fn as_fold() {
        let test: Option<String> = Some("Foo".into());

        assert_fold(Option::<String>::as_ref);
        assert_affine_fold(Option::<String>::as_ref);

        let mut iter = Option::as_ref.fold(&test);
        assert_eq!(iter.next().expect("some"), "Foo");
    }
}
