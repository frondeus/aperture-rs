use crate::{data::Person, prelude::*};

#[derive(Clone)]
pub struct PersonMotherAT;

impl AffineTraversal<Person> for PersonMotherAT {
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

impl AffineTraversalMut<Person> for PersonMotherAT {
    fn impl_set_mut<F>(&self, source: &mut Person, f: F)
    where
        F: Clone + FnMut(&mut Self::O),
    {
        let mut parents = source.parents.iter_mut();
        parents.next().map(f);
    }
}
impl AffineTraversalRef<Person> for PersonMotherAT {
    fn impl_preview_ref<'a>(&self, source: &'a Person) -> Option<&'a Self::O> {
        source.parents.first()
    }
}

#[derive(Clone)]
pub struct PersonParentsAT;

impl AffineTraversal<Person> for PersonParentsAT {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_mut() {
        let lens = PersonMotherAT;
        let mut wojtek = Person::wojtek();
        lens.set_mut(&mut wojtek, |mom| {
            mom.name = mom.name.to_uppercase();
        });
        let mom = &wojtek.parents[0].name;
        assert_eq!(mom, "MIROSLAWA");
    }

    #[test]
    fn as_ref() {
        let lens = PersonMotherAT;
        let wojtek = Person::wojtek();
        let mom = lens.map_opt_ref(&wojtek, |mom| mom.name.to_uppercase());
        assert_eq!(mom.unwrap(), "MIROSLAWA");

        let mom = lens
            .traverse_ref(&wojtek, |mom| mom.name.to_uppercase())
            .next();
        assert_eq!(mom.unwrap(), "MIROSLAWA");

        let mom = lens.fold_ref(&wojtek).next();
        assert_eq!(mom.unwrap().name, "Miroslawa");
    }

    #[test]
    fn as_af_ref() {
        let lens = PersonMotherAT;
        let wojtek = Person::wojtek();
        let mom = lens.preview_ref(&wojtek);
        assert_eq!(mom.unwrap().name, "Miroslawa");
    }
}
