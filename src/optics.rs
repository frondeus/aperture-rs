pub mod optics;
// 0 degree - Basic blocks
pub mod fold;
pub mod review;
pub mod setter;

// 1st degree
pub mod affine_fold;
pub mod traversal;

// 2nd degree
pub mod affine_traversal; // known as Optional
pub mod getter;

// 3rd degree - Complex
pub mod lens;
// mod rev_lens;
// mod prism;
// mod rev_prism;

// 4th degree
// mod iso;

// Combinators
pub mod impls;
pub mod then;

pub mod prelude {
    pub use crate::optics::{
        affine_fold::AffineFold,
        affine_traversal::AffineTraversal,
        fold::Fold,
        getter::Getter,
        impls::*,
        lens::{DerivedLens, Lens},
        optics::Optics,
        review::Review,
        setter::Setter,
        then::{And, Then},
        traversal::Traversal,
    };
}

#[cfg(test)]
mod tests {

    use crate::{
        data::{
            lenses::{PersonName, PersonParents},
            Person,
        },
        prelude::{every::Every, filtered::Filtered, *},
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
