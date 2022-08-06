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

pub trait Optics<As, S> {}
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
pub mod impls;
pub mod then;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use lenses_derive::*;

    pub use crate::{
        affine_fold::*,
        affine_traversal::*,
        fold::*,
        getter::*,
        impls::*,
        lens::*,
        prism::*,
        review::*,
        setter::*,
        then::*,
        traversal::*,
        Optics,
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        data::Person,
        prelude::{every::Every, *},
        setter::Setter,
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

        let mut iter = telescope.traverse_ref(&wojtek, |x| x.to_uppercase());
        assert_eq!(iter.next().unwrap(), "MIROSLAWA");
        assert_eq!(iter.next().unwrap(), "ZENON");
    }
}
