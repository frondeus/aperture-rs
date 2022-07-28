// #[derive(Clone)]
// pub struct PersonName;

// impl Lens<AsLens, Person> for PersonName {
//     type View = String;

//     fn impl_view(&self, source: Person) -> Self::View {
//         source.name
//     }

//     fn impl_set<F: FnMut(String) -> String>(&self, mut source: Person, mut f: F) -> Person {
//         source.name = f(source.name);
//         source
//     }
// }

// // pub type PersonMother2 = And<And<PersonParents, first::First>, unwrap::Unwrap>;

pub struct PersonMother;
impl Lens<AsLens, Person> for PersonMother {
    type View = Person;

    fn impl_view(&self, source: Person) -> Self::View {
        // Actually Person Mother should be a telescope
        source.parents.into_iter().next().unwrap()
    }

    fn impl_set<F: FnMut(Self::View) -> Self::View>(&self, mut source: Person, f: F) -> Person {
        let mut iter = source.parents.into_iter();
        let new_mom = iter.next().map(f);
        source.parents = new_mom.into_iter().chain(iter).collect();
        source
    }
}

// #[derive(Default, Debug)]
// pub struct PersonParents;

// impl Lens<AsLens, Person> for PersonParents {
//     type View = Vec<Person>;

//     fn impl_view(&self, source: Person) -> Self::View {
//         source.parents
//     }

//     fn impl_set<F: FnMut(Self::View) -> Self::View>(&self, mut source: Person, mut f: F) -> Person {
//         source.parents = f(source.parents);
//         source
//     }
// }
use crate::{
    data::{Person, *},
    prelude::*,
};
pub type PersonName = PersonNameLens;
pub type PersonParents = PersonParentsLens;

#[allow(non_upper_case_globals)]
impl Person {
    pub const mother: PersonMother = PersonMother;
    // pub const name: PersonName = PersonNameLens;
    // pub const parents: PersonParents = PersonParentsLens;
}

pub trait PersonLensesExt<S>: Lens<AsLens, S> + Sized {
    fn then_mother(self) -> And<Self, PersonMother, (AsLens, AsLens), (S, Person)> {
        self.then(PersonMother)
    }
    // fn then_name(self) -> And<Self, PersonName, (AsLens, AsLens), (S, Person)> {
    //     self.then(PersonNameLens)
    // }
}
impl<L, S> PersonLensesExt<S> for L where L: Lens<AsLens, S, View = Person> {}
