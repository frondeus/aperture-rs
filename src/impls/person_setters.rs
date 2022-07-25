use crate::{data::Person, prelude::*};

#[derive(Clone)]
pub struct PersonNameSetter;
impl Setter<AsSetter, Person> for PersonNameSetter {
    type O = String;

    type D = Person;

    type T = String;

    fn set<F>(&self, mut source: Person, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        source.name = f(source.name);
        source
    }
}

pub struct PersonMotherSetter;
impl Setter<AsSetter, Person> for PersonMotherSetter {
    type O = Person;

    type D = Person;

    type T = Person;

    fn set<F>(&self, mut source: Person, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        let mut parents = source.parents.into_iter();
        let mom = parents.next().map(f);
        source.parents = mom.into_iter().chain(parents).collect();
        source
    }
}

pub struct PersonParentsSetter;
impl Setter<AsSetter, Person> for PersonParentsSetter {
    type O = Vec<Person>;

    type D = Person;

    type T = Vec<Person>;

    fn set<F>(&self, mut source: Person, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        let parents = f(source.parents);
        source.parents = parents;
        source
    }
}
