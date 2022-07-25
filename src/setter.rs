use crate::prelude::*;

pub struct AsSetter;
pub trait Setter<As, S> {
    type O;
    type D;
    type T;
    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone;
}

impl<S, X> Optics<AsSetter, S> for X where X: Setter<AsSetter, S> {}
impl<L1, L2, S, T> Setter<AsSetter, S> for And<L1, L2, (AsSetter, AsSetter), (S, T)>
where
    L1: Setter<AsSetter, S, T = T, D = S, O = T>,
    L2: Setter<AsSetter, T, D = T>,
{
    type O = L2::O;

    type D = S;

    type T = L2::T;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        self.0.set(source, |o| self.1.set(o, f.clone()))
    }
}
impl<L1, L2, S, T> Setter<AsSetter, S> for And<L1, L2, (AsSetter, AsTraversal), (S, T)>
where
    L1: Setter<AsSetter, S, T = T, D = S, O = T>,
    L2: Setter<AsTraversal, T, D = T>,
{
    type O = L2::O;

    type D = S;

    type T = L2::T;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        self.0.set(source, |o| self.1.set(o, f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
            person_setters::{PersonMotherSetter, PersonNameSetter, PersonParentsSetter},
        },
    };

    #[test]
    fn setter_and_setter() {
        let lens = PersonMotherSetter.then(PersonNameSetter);

        let new = lens.set(Person::olivier(), |name| name.to_uppercase());
        assert_eq!(&new.parents[0].name, "ANNE");
    }

    #[test]
    fn and_is_valid_setter() {
        let lens = PersonMotherSetter
            .then(PersonMotherSetter)
            .then(PersonNameSetter);

        let new = lens.set(Person::wojtek(), |name| name.to_uppercase());
        assert_eq!(&new.parents[0].parents[0].name, "LIDIA");
    }

    #[test]
    fn setter_and_traversal() {
        let lens = PersonParentsSetter.then(Every);

        let new = lens.set(Person::olivier(), |mut parent| {
            parent.name = parent.name.to_uppercase();
            parent
        });

        assert_eq!(&new.parents[0].name, "ANNE");
        assert_eq!(&new.parents[1].name, "THIERRY");
    }
}
