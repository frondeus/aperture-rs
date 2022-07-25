use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsAffineFold;
pub trait AffineFold<As, S> // where
//     Self: Fold<As, S>,
{
    type T;
    fn preview(&self, source: S) -> Option<Self::T>;
}

impl<S, X> Optics<AsAffineFold, S> for X where X: AffineFold<AsAffineFold, S> {}
impl<As, X, S> Fold<(AsAffineFold, As), S> for X
where
    X: AffineFold<As, S>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

impl<L1, L2, S> AffineFold<AsAffineFold, S>
    for And<L1, L2, (AsAffineFold, AsAffineFold), (S, L1::T)>
where
    L1: AffineFold<AsAffineFold, S>,
    L2: AffineFold<AsAffineFold, L1::T>,
{
    type T = L2::T;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.0.preview(source).and_then(|t| self.1.preview(t))
    }
}

impl<L1, L2, S> Fold<AsFold, S> for And<L1, L2, (AsAffineFold, AsFold), (S, L1::T)>
where
    L1: AffineFold<AsAffineFold, S>,
    L2: Fold<AsFold, L1::T>,
    L2::D: Iterator,
{
    type D = std::iter::Flatten<std::option::IntoIter<L2::D>>;

    fn fold(&self, source: S) -> Self::D {
        self.0
            .preview(source)
            .map(|t| self.1.fold(t))
            .into_iter()
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
            list_of::ListOf,
            person_af::{PersonMotherAF, PersonParentsAF},
            person_setters::PersonNameSetter,
        },
    };

    #[test]
    fn af_and_af() {
        let lens = PersonMotherAF.then(PersonMotherAF);

        let grandma = lens.preview(Person::wojtek());
        assert_eq!(grandma.unwrap().name, "Lidia");
    }

    #[test]
    fn and_is_valid_af() {
        let lens = PersonMotherAF.then(PersonMotherAF).then(PersonMotherAF);

        let grand_grandma = lens.preview(Person::wojtek());
        assert_eq!(grand_grandma, None);
    }

    // #[test]
    // fn af_and_setter() {
    //     let lens = PersonMotherAF.then(PersonNameSetter);

    //     let mom = lens.set(Person::wojtek(), |name| name.to_uppercase());
    //     assert_eq!(mom.unwrap().name, "MIROSLAWA");
    // }

    #[test]
    fn af_and_fold() {
        let lens = PersonParentsAF.then(ListOf);

        let mut parents = lens.fold(Person::wojtek());
        assert_eq!(parents.next().unwrap().name, "Miroslawa");
        assert_eq!(parents.next().unwrap().name, "Zenon");
    }

    // #[test]
    // fn af_and_traversal() {
    //     let lens = PersonParentsAF.then(Every);

    //     let mut parents = lens.traverse(Person::wojtek(), |x| x);
    //     assert_eq!(parents.next().unwrap().name, "Miroslawa");
    //     assert_eq!(parents.next().unwrap().name, "Zenon");

    //     let wojtek = lens.set(Person::wojtek(), |mut x| {
    //         x.name = x.name.to_uppercase();
    //         x
    //     });
    //     assert_eq!(wojtek.parents[0].name, "MIROSLAWA");
    //     assert_eq!(wojtek.parents[1].name, "ZENON");
    // }
}
