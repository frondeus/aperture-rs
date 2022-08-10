// #![feature(unboxed_closures, fn_traits, type_alias_impl_trait)]
#![cfg_attr(feature = "gat", feature(generic_associated_types))]
// #![allow(unused_imports)]
#![deny(clippy::all)]
// #![deny(clippy::pedantic)]

// mod sealed {
//     pub trait Sealed<S, A> {}
// }
// pub mod identity;
// pub mod lazy;
// pub mod method;

#[cfg(test)]
mod data;

pub trait Optics<As: Default + ::std::fmt::Debug, S> {
    fn is_optics(&self) -> As {
        As::default()
    }
}
// 0 degree - Basic blocks
pub mod fold; // 23 = 1 + 11 * 2
pub mod setter; // 11 = 1 + 5 * 2

// 1st degree
pub mod affine_fold; // 21 = 1 + 10 * 2
pub mod traversal; // 9 = 1 + 4 * 2

// 2nd degree
pub mod affine_traversal; // known as Optional 9 = 1 + 4 * 2
pub mod getter; // 9 = 1 + 4 * 2
pub mod review; // 9 = 1 + 4 * 2

// 3rd degree - Complex
pub mod lens; // 3 = 1 + 1 * 2
pub mod prism; // 3

// pub mod rev_lens; // 3
// pub mod rev_prism; // 3

// 4th degree
// pub mod iso; // 1

// Combinators
#[cfg(test)]
pub mod impls;
pub mod std;
pub mod then;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use lenses_derive::*;

    #[cfg(test)]
    pub use crate::impls::*;
    pub use crate::{
        affine_fold::*,
        affine_traversal::*,
        fold::*,
        getter::*,
        lens::*,
        prism::*,
        review::*,
        setter::*,
        std::{_Err, _None, _Ok, _Some},
        then::*,
        traversal::*,
        Optics,
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{Person, SomeNestedStructure, SomeStructure},
        prelude::*,
        setter::Setter,
        std::{Every, _Err, _Ok},
    };

    #[test]
    fn example() {
        let telescope = Person::parents.then(Every).then(Person::name);

        let mut wojtek = telescope.set(Person::wojtek(), |x| x.to_uppercase());
        assert_eq!(wojtek.parents[0].name, "MIROSLAWA");
        assert_eq!(wojtek.parents[1].name, "ZENON");

        telescope.set_mut(&mut wojtek, |x| *x = x.to_lowercase());
        assert_eq!(wojtek.parents[0].name, "miroslawa");
        assert_eq!(wojtek.parents[1].name, "zenon");

        let marker = telescope.is_optics();
        assert_eq!(marker, AsTraversal);

        let mut iter = telescope.fold_ref(&wojtek);
        assert_eq!(iter.next().unwrap(), "miroslawa");
        assert_eq!(iter.next().unwrap(), "zenon");

        let mut iter = telescope.fold(wojtek);
        assert_eq!(iter.next().unwrap(), "miroslawa");
        assert_eq!(iter.next().unwrap(), "zenon");
    }

    #[test]
    fn example_2() {
        let test = SomeNestedStructure::test();

        let telescope = SomeNestedStructure::inner
            .then(Every)
            .then(SomeStructure::person_opt)
            .then(_Some)
            .then(Person::name);

        let mut names = telescope.fold_ref(&test);
        assert_eq!(names.next().unwrap(), "Olivier");
        assert_eq!(names.next(), None);

        let telescope = SomeNestedStructure::inner
            .then(Every)
            .then(SomeStructure::person_res)
            .then(_Err);

        let mut errors = telescope.fold_ref(&test);
        assert_eq!(errors.next().unwrap(), "String");
        assert_eq!(errors.next(), None);

        let telescope = SomeNestedStructure::inner
            .then(Every)
            .then(SomeStructure::person_res)
            .then(_Ok)
            .then(Person::name);

        fn impl_part<As, S, T>(telescope: T, test: &S)
        where
            for<'a> T: FoldRef<As, S, Item<'a> = String>,
            T::D: Iterator<Item = String>,
        {
            // Note, that the function does not know from where the data comes from,
            // what is data structure, nor how the telescope looks like
            // All it cares that the telescope returns an iterator of &String.
            let mut errors = telescope.fold_ref(test);
            assert_eq!(errors.next().unwrap(), "Wojtek");
            assert_eq!(errors.next(), None);
        }
        impl_part(telescope, &test)
    }
}
