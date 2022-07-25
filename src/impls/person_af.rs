use crate::{
    data::Person,
    prelude::{AffineFold, *},
};

#[derive(Clone)]
pub struct PersonMotherAF;

impl AffineFold<AsAffineFold, Person> for PersonMotherAF {
    type T = Person;

    fn preview(&self, source: Person) -> Option<Self::T> {
        source.parents.into_iter().next()
    }
}

#[derive(Clone)]
pub struct PersonParentsAF;

impl AffineFold<AsAffineFold, Person> for PersonParentsAF {
    type T = Vec<Person>;

    fn preview(&self, source: Person) -> Option<Self::T> {
        Some(source.parents)
    }
}
