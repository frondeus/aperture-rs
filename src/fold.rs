use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsFold;

pub trait Fold<S, As = AsFold> {
    type D;

    fn fold(&self, source: S) -> Self::D;
}

pub trait FoldRef<S, As = AsFold>
where
    Self: Fold<S, As>,
    Self::D: Iterator,
{
    type Item<'a>: 'a
    where
        S: 'a;
    type DRef<'a>: Iterator<Item = &'a Self::Item<'a>>
    where
        S: 'a;

    fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a>;
}

mod nested;
pub use nested::*;

impl<S, X> Optics<S, AsFold> for X where X: Fold<S> {}
// macro_rules! impl_and {
// ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
// (@ $(($l:ident, $r:ident)),*) => {$(
// )*};
// }
macro_rules! impl_fold {
    ($as: ident, $(($l:ident, $r:ident),)*) => { impl_fold!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
    (@ $(($l:ident, $r:ident)),*) => {$(
        impl<L1, L2, S> Fold<S> for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
        where
            L1: Fold<S, $l>,
            L1::D: Iterator,
            L2: Fold<<L1::D as Iterator>::Item, $r>,
            L2: Clone,
            L2::D: Iterator,
        {
            type D = nested::NestedFold<$r, L1::D, L2>;

            fn fold(&self, source: S) -> Self::D {
                nested::NestedFold::new(self.0.fold(source), self.1.clone())
            }
        }
        impl<L1, L2, S> FoldRef<S> for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
        where
            L1: Fold<S, $l>,
            L1::D: Iterator,
            L2: Fold<<L1::D as Iterator>::Item, $r>,
            L2: Clone,
            <L2 as Fold<<L1::D as Iterator>::Item, $r>>::D: Iterator,

            L1: FoldRef<S, $l>,
            for<'a> L2: FoldRef< L1::Item<'a>, $r>,
            for<'a> <L2 as Fold<L1::Item<'a>, $r>>::D: Iterator,
            for<'a> S: 'a
        {
            type Item<'a> = <L2 as FoldRef<L1::Item<'a>, $r>>::Item<'a>;

            type DRef<'a> = nested::NestedFoldRef<
                'a,
                $l,
                $r,
                L1,
                L2,
                S
            >;

            fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
                nested::NestedFoldRef::new(self.0.fold_ref(source), self.1.clone())
            }
        }

    )*};
}

impl_fold!(
    AsFold,
    (AsFold, AsTraversal),
    (AsFold, AsAffineFold),
    (AsFold, AsAffineTraversal),
    (AsFold, AsGetter),
    (AsFold, AsLens),
    // (AsFold, AsRevPrism),
    (AsFold, AsPrism),
    // (AsFold, AsIso)
    (AsTraversal, AsAffineFold),
    (AsTraversal, AsGetter),
    // (AsTraversal, AsRevPrism),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            person_af::PersonMotherAF,
            person_at::PersonMotherAT,
            person_folds::{PersonGrandParentsFold, PersonParentsFold},
        },
        std::Every,
    };

    #[test]
    fn fold_and_fold() {
        let grandparents = PersonParentsFold.then(PersonParentsFold);

        let mut iter = grandparents.fold(Person::wojtek());
        assert_eq!(iter.next().unwrap().name, "Lidia");
        assert_eq!(iter.next().unwrap().name, "Jerzy");
        assert_eq!(iter.next().unwrap().name, "Helena");
        assert_eq!(iter.next().unwrap().name, "Waclaw");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fold_and_fold_ref() {
        let grandparents = PersonParentsFold.then(PersonParentsFold);

        let wojtek = Person::wojtek();
        let mut iter = grandparents.fold_ref(&wojtek);
        assert_eq!(iter.next().unwrap().name, "Lidia");
        assert_eq!(iter.next().unwrap().name, "Jerzy");
        assert_eq!(iter.next().unwrap().name, "Helena");
        assert_eq!(iter.next().unwrap().name, "Waclaw");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn and_is_valid_fold() {
        let grandparents = PersonParentsFold
            .then(PersonParentsFold)
            .then(PersonParentsFold);

        let mut iter = grandparents.fold(Person::wojtek());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fold_and_traversal() {
        let parents = PersonGrandParentsFold.then(Every);
        let mut iter = parents.fold(Person::wojtek());
        assert_eq!(iter.next().unwrap().name, "Lidia");
        assert_eq!(iter.next().unwrap().name, "Jerzy");
        assert_eq!(iter.next().unwrap().name, "Helena");
        assert_eq!(iter.next().unwrap().name, "Waclaw");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fold_and_af() {
        let parents = PersonParentsFold.then(PersonMotherAF);
        let mut iter = Fold::fold(&parents, Person::wojtek());
        assert_eq!(iter.next().unwrap().name, "Lidia");
        assert_eq!(iter.next().unwrap().name, "Helena");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fold_and_at() {
        let parents = PersonParentsFold.then(PersonMotherAT);
        let mut iter = Fold::fold(&parents, Person::wojtek());
        assert_eq!(iter.next().unwrap().name, "Lidia");
        assert_eq!(iter.next().unwrap().name, "Helena");
        assert_eq!(iter.next(), None);
    }
}
