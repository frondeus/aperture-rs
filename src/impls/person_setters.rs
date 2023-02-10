use crate::{data::Person, prelude::*};

#[derive(Clone)]
pub struct PersonNameSetter;
impl Setter<Person> for PersonNameSetter {
    type O = String;

    fn set<F>(&self, mut source: Person, mut f: F) -> Person
    where
        F: FnMut(Self::O) -> Self::O + Clone,
    {
        source.name = f(source.name);
        source
    }
}
impl SetterMut<Person> for PersonNameSetter {
    fn set_mut<F>(&self, source: &mut Person, mut f: F)
    where
        F: FnMut(&mut Self::O) + Clone,
    {
        f(&mut source.name)
    }
}

pub struct PersonMotherSetter;
impl Setter<Person> for PersonMotherSetter {
    type O = Person;

    fn set<F>(&self, mut source: Person, f: F) -> Person
    where
        F: FnMut(Self::O) -> Self::O + Clone,
    {
        let mut parents = source.parents.into_iter();
        let mom = parents.next().map(f);
        source.parents = mom.into_iter().chain(parents).collect();
        source
    }
}
impl SetterMut<Person> for PersonMotherSetter {
    fn set_mut<F>(&self, source: &mut Person, f: F)
    where
        F: FnMut(&mut Self::O) + Clone,
    {
        source.parents.iter_mut().next().map(f);
    }
}

pub struct PersonParentsSetter;
impl Setter<Person> for PersonParentsSetter {
    type O = Vec<Person>;

    fn set<F>(&self, mut source: Person, mut f: F) -> Person
    where
        F: FnMut(Self::O) -> Self::O + Clone,
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
        PersonNameSetter.set_mut(&mut wojtek, |x| *x = x.to_uppercase());
        assert_eq!(wojtek.name, "WOJTEK");
    }
}
