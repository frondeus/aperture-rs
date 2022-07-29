use crate::{data::Person, prelude::*};

impl<'a> Setter<AsSetter, &'a mut Person> for PersonNameSetter {
    type O = &'a mut String;

    type D = ();

    type T = ();

    fn set<F>(&self, source: &'a mut Person, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        f(&mut source.name);
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set() {
        let wojtek = PersonNameSetter.set(Person::wojtek(), |x| x.to_uppercase());
        assert_eq!(wojtek.name, "WOJTEK");
    }

    #[test]
    fn set_mut() {
        let mut wojtek = Person::wojtek();
        PersonNameSetter.set(&mut wojtek, |x| *x = x.to_uppercase());
        assert_eq!(wojtek.name, "WOJTEK");
    }
}
