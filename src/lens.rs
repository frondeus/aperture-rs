// mod tuple;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsLens;
pub trait Lens<As, S> {
    type View;
    fn impl_view(&self, source: S) -> Self::View;
    fn impl_set<F: Clone + FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S;
}

impl<As, X, S> Getter<(AsLens, As), S> for X
where
    X: Lens<As, S>,
{
    type T = X::View;

    fn view(&self, source: S) -> <Self as Getter<(AsLens, As), S>>::T {
        self.impl_view(source)
    }
}

impl<As, X, S> AffineTraversal<(AsLens, As), S> for X
where
    X: Lens<As, S>,
{
    type O = X::View;

    fn impl_preview(&self, source: S) -> Option<Self::O> {
        Some(self.impl_view(source))
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O,
    {
        Lens::impl_set(self, source, f)
    }
}

impl<L1, L2, S> Lens<AsLens, S> for And<L1, L2>
where
    L1: Lens<AsLens, S>,
    L2: Lens<AsLens, L1::View>,
{
    type View = L2::View;

    fn impl_view(&self, source: S) -> Self::View {
        self.1.view(self.0.view(source))
    }

    fn impl_set<F: Clone + FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S {
        self.0.set(source, |p| self.1.set(p, f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::person_lenses::{PersonMother, PersonName, PersonParents},
    };

    #[test]
    fn lens_and_lens() {
        let lens = PersonMother.then(PersonName);

        let name = lens.view(Person::olivier());
        assert_eq!(name, "Anne");

        let olivier = lens.set(Person::olivier(), |name| name.to_uppercase());
        assert_eq!(olivier.parents[0].name, "ANNE");
    }

    #[test]
    fn as_getter() {
        let mom = PersonMother.view(Person::olivier());
        assert_eq!(&mom.name, "Anne");

        let parents = PersonParents.view(Person::olivier());
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents[1].name, "Thierry");
    }

    #[test]
    fn as_affine_fold() {
        let mom: Option<Person> = PersonMother.preview(Person::olivier());
        assert_eq!(mom.unwrap().name, "Anne");

        let parents = PersonParents.preview(Person::olivier());
        let parents = parents.unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents[1].name, "Thierry");
    }

    #[test]
    fn as_fold() {
        let mut mom = PersonMother.fold(Person::olivier());
        assert_eq!(mom.next().unwrap().name, "Anne");

        let mut parents = PersonParents.fold(Person::olivier());
        let parents = parents.next().unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents[1].name, "Thierry");
    }

    #[test]
    fn as_affine_traversal() {
        let mom = PersonMother.map_opt(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        assert_eq!(mom.unwrap().name, "Jocelyn");

        let parents = PersonParents.map_opt(Person::olivier(), |mut parents| {
            parents.pop();
            parents
        });
        let parents = parents.unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents.len(), 1);
    }

    #[test]
    fn as_traversal() {
        let mut mom = PersonMother.traverse(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        assert_eq!(mom.next().unwrap().name, "Jocelyn");

        let mut parents = PersonParents.traverse(Person::olivier(), |mut parents| {
            parents.pop();
            parents
        });
        let parents = parents.next().unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents.len(), 1);
    }

    #[test]
    fn as_setter() {
        let new_olivier = PersonMother.set(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        assert_eq!(new_olivier.parents[0].name, "Jocelyn");

        let new_olivier = PersonParents.set(Person::olivier(), |mut parents| {
            parents.pop();
            parents
        });
        assert_eq!(new_olivier.parents[0].name, "Anne");
        assert_eq!(new_olivier.parents.len(), 1);
    }
}
