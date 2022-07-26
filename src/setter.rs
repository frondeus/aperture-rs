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

macro_rules! impl_setter {
    ($as: ident, $(($l:ident, $r:ident),)*) => { impl_setter!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
    (@ $(($l:ident, $r:ident)),*) => {$(
        impl<L1, L2, S, T> Setter<AsSetter, S> for And<L1, L2, ($l, $r), (S, T)>
        where
            L1: Setter<$l, S, T = T, D = S, O = T>,
            L2: Setter<$r, T, D = T>,
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
    )*};
}

impl_setter!(
    AsSetter,
    (AsSetter, AsTraversal),
    (AsSetter, AsAffineTraversal),
    (AsSetter, AsLens),
    // ( AsSetter, AsPrism ),
    // ( AsSetter, AsIso ),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
            person_af::PersonMotherAF,
            person_at::PersonMotherAT,
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

    #[test]
    fn setter_and_at() {
        let lens = PersonMotherSetter.then(PersonMotherAT);

        let new = lens.set(Person::wojtek(), |mut parent| {
            parent.name = parent.name.to_uppercase();
            parent
        });

        assert_eq!(&new.parents[0].name, "Miroslawa");
        assert_eq!(&new.parents[0].parents[0].name, "LIDIA");
        assert_eq!(&new.parents[0].parents[1].name, "Jerzy");
        assert_eq!(&new.parents[1].name, "Zenon");
    }

    // #[test]
    // fn setter_and_af() {
}
