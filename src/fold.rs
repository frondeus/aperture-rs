use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsFold;
pub trait Fold<As, S> {
    type D;

    fn fold(&self, source: S) -> Self::D;
}

mod nested;
pub use nested::*;

impl<S, X> Optics<AsFold, S> for X where X: Fold<AsFold, S> {}
impl<L1, L2, S> Fold<AsFold, S> for And<L1, L2, (AsFold, AsFold), (S, <L1::D as Iterator>::Item)>
where
    L1: Fold<AsFold, S>,
    L1::D: Iterator,
    L2: Fold<AsFold, <L1::D as Iterator>::Item>,
    L2: Clone,
    L2::D: Iterator,
{
    type D = nested::NestedFold<AsFold, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        nested::NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

impl<L1, L2, S> Fold<AsFold, S>
    for And<L1, L2, (AsFold, AsTraversal), (S, <L1::D as Iterator>::Item)>
where
    L1: Fold<AsFold, S>,
    L1::D: Iterator,
    L2: Fold<AsTraversal, <L1::D as Iterator>::Item>,
    L2: Clone,
    L2::D: Iterator,
{
    type D = nested::NestedFold<AsTraversal, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        nested::NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

impl<L1, L2, S> Fold<AsFold, S>
    for And<L1, L2, (AsFold, AsAffineFold), (S, <L1::D as Iterator>::Item)>
where
    L1: Fold<AsFold, S>,
    L1::D: Iterator,
    L2: Fold<AsAffineFold, <L1::D as Iterator>::Item>,
    L2: Clone,
    L2::D: Iterator,
{
    type D = nested::NestedFold<AsAffineFold, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        nested::NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

impl<L1, L2, S> Fold<AsFold, S>
    for And<L1, L2, (AsFold, AsAffineTraversal), (S, <L1::D as Iterator>::Item)>
where
    L1: Fold<AsFold, S>,
    L1::D: Iterator,
    L2: Fold<AsAffineTraversal, <L1::D as Iterator>::Item>,
    L2: Clone,
    L2::D: Iterator,
{
    type D = nested::NestedFold<AsAffineTraversal, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        nested::NestedFold::new(self.0.fold(source), self.1.clone())
    }
}
// impl<S, M> Fold<sFold, S> for M where M: crate::method::Method<S, ()> {
//     type D;

//     fn fold(&self, source: S) -> Self::D {
//         todo!()
//     }
// }

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
