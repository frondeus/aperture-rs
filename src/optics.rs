// 0 degree - Basic blocks
mod setter;
pub use setter::*;

mod review;
pub use review::*;

mod fold;
pub use fold::*;

// 1st degree
mod affine_fold;
pub use affine_fold::*;

mod traversal;
pub use traversal::*;

// 2nd degree
mod affine_traversal; // known as Optional
pub use affine_traversal::*;

mod getter;
pub use getter::*;

// 3rd degree - Complex
mod lens;
pub use lens::*;
// mod rev_lens;
// mod prism;
// mod rev_prism;

// 4th degree
// mod iso;

// Combinators
mod then;
pub use then::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            lenses::{PersonName, PersonParents},
            Person,
        },
        optics::{Filtered, Fold},
    };

    #[test]
    fn collections() {
        let lens = PersonParents.then(Every);
        let mut parents = lens.fold(Person::olivier());
        assert_eq!(parents.next().unwrap().name, "Anne");
        assert_eq!(parents.next().unwrap().name, "Thierry");
        assert_eq!(parents.next(), None);
    }

    #[test]
    fn long_collections() {
        let lens = PersonParents.then(Every).then(PersonName);
        let mut parents = lens.fold(Person::olivier());
        assert_eq!(parents.next().unwrap(), "Anne");
        assert_eq!(parents.next().unwrap(), "Thierry");
        assert_eq!(parents.next(), None);

        let lens = PersonParents
            .then(Filtered(|person: &Person| person.age < 56))
            .then(PersonName);
        let mut parents = lens.fold(Person::olivier());
        assert_eq!(parents.next().unwrap(), "Anne");
        assert_eq!(parents.next(), None);

        let lens = PersonParents
            .then(Filtered(|person: &Person| person.age > 55))
            .then(PersonName);
        let mut parents = lens.fold(Person::olivier());
        assert_eq!(parents.next().unwrap(), "Thierry");
        assert_eq!(parents.next(), None);

        let lens = PersonParents
            .then(Filtered(|person: &Person| person.age > 55))
            .then(PersonName);
        let new_olivier = lens.set(Person::olivier(), |_t| "Mark".to_string());

        assert_eq!(new_olivier.parents[0].name, "Anne");
        assert_eq!(new_olivier.parents[1].name, "Mark");
    }
}
