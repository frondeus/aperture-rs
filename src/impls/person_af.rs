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

impl<'a> AffineFold<AsAffineFold, &'a Person> for PersonMotherAF {
    type T = &'a Person;

    fn preview(&self, source: &'a Person) -> Option<Self::T> {
        source.parents.first()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn af() {
        let mother = PersonMotherAF.preview(Person::wojtek());
        assert_eq!(mother.unwrap().name, "Miroslawa");
    }

    #[test]
    fn af_ref() {
        let wojtek = Person::wojtek();
        let mother = PersonMotherAF.preview(&wojtek);
        assert_eq!(mother.unwrap().name, "Miroslawa");
    }
}
