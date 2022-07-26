use crate::{data::Person, prelude::*};

#[derive(Clone)]
pub struct PersonMotherAT;

impl AffineTraversal<AsAffineTraversal, Person> for PersonMotherAT {
    type O = Person;

    fn impl_preview(&self, source: Person) -> Option<Self::O> {
        source.parents.into_iter().next()
    }

    fn impl_set<F>(&self, mut source: Person, f: F) -> Person
    where
        F: Clone + FnMut(Self::O) -> Self::O,
    {
        let mut parents = source.parents.into_iter();
        let mom = parents.next().map(f);
        source.parents = mom.into_iter().chain(parents).collect();
        source
    }
}

#[derive(Clone)]
pub struct PersonParentsAT;

impl AffineTraversal<AsAffineTraversal, Person> for PersonParentsAT {
    type O = Vec<Person>;

    fn impl_preview(&self, source: Person) -> Option<Self::O> {
        Some(source.parents)
    }

    fn impl_set<F>(&self, mut source: Person, mut f: F) -> Person
    where
        F: Clone + FnMut(Self::O) -> Self::O,
    {
        source.parents = f(source.parents);
        source
    }
}
