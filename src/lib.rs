// #![feature(unboxed_closures, fn_traits, type_alias_impl_trait)]
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

// WIP
mod prism; // 3
           // mod rev_lens; // 3
           // mod rev_prism; // 3

// 4th degree
// mod iso; // 1

// Combinators
pub mod impls;
pub mod then;

pub mod prelude {
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
        prelude::{
            every::Every,
            person_lenses::{PersonName, PersonParents},
            Then,
        },
        setter::Setter,
    };

    #[test]
    fn example() {
        let telescope = PersonParents.then(Every).then(PersonName);

        let wojtek = telescope.set(Person::wojtek(), |x| x.to_uppercase());
        assert_eq!(wojtek.parents[0].name, "MIROSLAWA");
        assert_eq!(wojtek.parents[1].name, "ZENON");
    }
    //     use crate::{
    //         data::{
    //             lenses::{PersonName, PersonParents},
    //             Person,
    //         },
    //         prelude::{every::Every, filtered::Filtered, *},
    //     };

    //     #[test]
    //     fn collections() {
    //         let lens = PersonParents.then(Every);
    //         let mut parents = lens.fold(Person::olivier());
    //         assert_eq!(parents.next().unwrap().name, "Anne");
    //         assert_eq!(parents.next().unwrap().name, "Thierry");
    //         assert_eq!(parents.next(), None);
    //     }

    //     #[test]
    //     fn long_collections() {
    //         let lens = PersonParents.then(Every).then(PersonName);
    //         let mut parents = lens.fold(Person::olivier());
    //         assert_eq!(parents.next().unwrap(), "Anne");
    //         assert_eq!(parents.next().unwrap(), "Thierry");
    //         assert_eq!(parents.next(), None);

    //         let lens = PersonParents
    //             .then(Filtered(|person: &Person| person.age < 56))
    //             .then(PersonName);
    //         let mut parents = lens.fold(Person::olivier());
    //         assert_eq!(parents.next().unwrap(), "Anne");
    //         assert_eq!(parents.next(), None);

    //         let lens = PersonParents
    //             .then(Filtered(|person: &Person| person.age > 55))
    //             .then(PersonName);
    //         let mut parents = lens.fold(Person::olivier());
    //         assert_eq!(parents.next().unwrap(), "Thierry");
    //         assert_eq!(parents.next(), None);

    //         let lens = PersonParents
    //             .then(Filtered(|person: &Person| person.age > 55))
    //             .then(PersonName);

    //         let new_olivier = lens.set(Person::olivier(), |_t| "Mark".to_string());

    //         assert_eq!(new_olivier.parents[0].name, "Anne");
    //         assert_eq!(new_olivier.parents[1].name, "Mark");
    //     }
}
