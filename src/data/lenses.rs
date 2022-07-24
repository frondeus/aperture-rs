use super::Person;
use crate::prelude::*;

#[derive(Clone)]
pub struct PersonName;

impl DerivedLens<Person> for PersonName {
    type View = String;

    fn derived_view(&self, source: Person) -> Self::View {
        source.name
    }

    fn derived_set<F: FnMut(String) -> String>(&self, mut source: Person, mut f: F) -> Person {
        source.name = f(source.name);
        source
    }
}

pub type PersonMother2 = And<And<PersonParents, first::First>, unwrap::Unwrap>;

pub struct PersonMother;
impl DerivedLens<Person> for PersonMother {
    type View = Person;

    fn derived_view(&self, source: Person) -> Self::View {
        // Actually Person Mother should be a telescope
        source.parents.into_iter().next().unwrap()
    }

    fn derived_set<F: FnMut(Self::View) -> Self::View>(&self, mut source: Person, f: F) -> Person {
        let mut iter = source.parents.into_iter();
        let new_mom = iter.next().map(f);
        source.parents = new_mom.into_iter().chain(iter).collect();
        source
    }
}

#[derive(Default, Debug)]
pub struct PersonParents;

impl DerivedLens<Person> for PersonParents {
    type View = Vec<Person>;

    fn derived_view(&self, source: Person) -> Self::View {
        source.parents
    }

    fn derived_set<F: FnMut(Self::View) -> Self::View>(
        &self,
        mut source: Person,
        mut f: F,
    ) -> Person {
        source.parents = f(source.parents);
        source
    }
}
