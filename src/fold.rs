use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsFold;
pub trait Fold<As, S> {
    type D;

    fn fold(&self, source: S) -> Self::D;
}

// #[cfg(not(feature = "gat"))]
// pub trait FoldRef<'a, As, S>
// where
//     Self: Fold<As, S>,
//     Self::D: Iterator,
// {
//     type DRef: 'a;
//     fn fold_ref(&self, source: &'a S) -> Self::DRef;
// }

// #[cfg(feature = "gat")]
pub trait FoldRef<As, S>
where
    Self: Fold<As, S>,
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

impl<S, X> Optics<AsFold, S> for X where X: Fold<AsFold, S> {}
// macro_rules! impl_and {
// ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
// (@ $(($l:ident, $r:ident)),*) => {$(
// )*};
// }
macro_rules! impl_fold {
    ($as: ident, $(($l:ident, $r:ident),)*) => { impl_fold!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
    (@ $(($l:ident, $r:ident)),*) => {$(
        impl<L1, L2, S> Fold<AsFold, S> for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
        where
            L1: Fold<$l, S>,
            L1::D: Iterator,
            L2: Fold<$r, <L1::D as Iterator>::Item>,
            L2: Clone,
            L2::D: Iterator,
        {
            type D = nested::NestedFold<$r, L1::D, L2>;

            fn fold(&self, source: S) -> Self::D {
                nested::NestedFold::new(self.0.fold(source), self.1.clone())
            }
        }
        impl<L1, L2, S> FoldRef<AsFold, S> for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
        where
            L1: Fold<$l, S>,
            L1::D: Iterator,
            L2: Fold<$r, <L1::D as Iterator>::Item>,
            L2: Clone,
            <L2 as Fold<$r, <L1::D as Iterator>::Item>>::D: Iterator,

            L1: FoldRef<$l, S>,
            for<'a> L2: FoldRef<$r, L1::Item<'a>>,
            for<'a> <L2 as Fold<$r, L1::Item<'a> >>::D: Iterator,
            for<'a> S: 'a
        {
            type Item<'a> = <L2 as FoldRef<$r, L1::Item<'a>>>::Item<'a>;

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
            every::Every,
            person_af::PersonMotherAF,
            person_at::PersonMotherAT,
            person_folds::{PersonGrandParentsFold, PersonParentsFold},
        },
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
