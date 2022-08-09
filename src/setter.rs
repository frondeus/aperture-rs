use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsSetter;
pub trait Setter<As, S> {
    type O;
    fn set<F>(&self, source: S, f: F) -> S
    where
        F: FnMut(Self::O) -> Self::O + Clone;
}

// If FnMut: FnOnce then SetterMut: Setter
pub trait SetterMut<As, S>: Setter<As, S> {
    fn set_mut<F>(&self, source: &mut S, f: F)
    where
        F: FnMut(&mut Self::O) + Clone;
}

pub trait PSetter<As, S> {
    type O;
    type D;
    type T;
    fn pset<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone;
}

impl<X, As, S> PSetter<As, S> for X
where
    X: Setter<As, S>,
{
    type O = X::O;

    type D = S;

    type T = X::O;

    fn pset<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        self.set(source, f)
    }
}

impl<S, X> Optics<AsSetter, S> for X where X: Setter<AsSetter, S> {}
// impl<S, X> Optics<AsSetter, S> for X where X: SetterMut<AsSetter, S> {}

macro_rules! impl_setter {
    ($as: ident, $(($l:ident, $r:ident),)*) => { impl_setter!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
    (@ $(($l:ident, $r:ident)),*) => {$(
        impl<L1, L2, S, T> Setter<AsSetter, S> for And<L1, L2, ($l, $r), (S, T)>
        where
            L1: Setter<$l, S, O = T>,
            L2: Setter<$r, T>,
        {
            type O = L2::O;

            fn set<F>(&self, source: S, f: F) -> S
            where
                F: FnMut(Self::O) -> Self::O + Clone,
            {
                self.0.set(source, |o| self.1.set(o, f.clone()))
            }
        }
        impl<L1, L2, S, T> SetterMut<AsSetter, S> for And<L1, L2, ($l, $r), (S, T)>
        where
            L1: SetterMut<$l, S> + Setter<$l, S, O = T>,
            L2: SetterMut<$r, T>,
        {
            fn set_mut<F>(&self, source: &mut S, f: F)
            where
                F: FnMut(&mut Self::O) + Clone,
            {
                self.0.set_mut(source, |o| self.1.set_mut(o, f.clone()))
            }
        }
    )*};
}

impl_setter!(
    AsSetter,
    (AsSetter, AsTraversal),
    (AsSetter, AsAffineTraversal),
    (AsSetter, AsLens),
    (AsSetter, AsPrism),
    // ( AsSetter, AsIso ),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
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
    fn setter_and_setter_mut() {
        let lens = PersonMotherSetter.then(PersonNameSetter);

        let mut olivier = Person::olivier();
        lens.set_mut(&mut olivier, |name| *name = name.to_uppercase());
        assert_eq!(&olivier.parents[0].name, "ANNE");
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
