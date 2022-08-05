use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsAffineFold;
pub trait AffineFold<As, S> {
    type T;
    fn preview(&self, source: S) -> Option<Self::T>;
}

pub trait AffineFoldRef<As, S>: AffineFold<As, S> {
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T>;
}

impl<S, X> Optics<AsAffineFold, S> for X where X: AffineFold<AsAffineFold, S> {}
impl<X, S> Fold<AsAffineFold, S> for X
where
    X: AffineFold<AsAffineFold, S>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

#[cfg(feature = "gat")]
impl<X, S> FoldRef<AsAffineFold, S> for X
where
    X: AffineFoldRef<AsAffineFold, S>,
    for<'a> X::T: 'a,
    for<'a> S: 'a,
{
    type Item<'a> = X::T;

    type DRef<'a> = std::option::IntoIter<&'a X::T>;

    fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.preview_ref(source).into_iter()
    }
}

impl<X, S, T> AffineFoldRef<AsAffineFold, S> for X
where
    X: for<'b> AffineFold<AsAffineFold, &'b S, T = &'b T>,
    X: AffineFold<AsAffineFold, S, T = T>,
{
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.preview(source)
    }
}

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
 (@ $(($l:ident, $r:ident)),*) => {$(
impl<L1, L2, S> AffineFold<AsAffineFold, S>
    for And<L1, L2, ($l, $r), (S, L1::T)>
where
    L1: AffineFold<$l, S>,
    L2: AffineFold<$r, L1::T>,
{
    type T = L2::T;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.0.preview(source).and_then(|t| self.1.preview(t))
    }
}
 )*};
}

impl_and!(
    AsAffineFold,
    (AsAffineFold, AsAffineTraversal),
    (AsAffineFold, AsGetter),
    (AsAffineFold, AsLens),
    (AsAffineTraversal, AsGetter),
    // (AsAffineTraversal, AsRevPrism),
    // (AsGetter, AsRevPrism),
    // (AsRevPrism, AsPrism),
    // (AsAffineFold, AsRevPrism),
    (AsAffineFold, AsPrism),
    // (AsAffineFold, AsIso),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
            list_of::ListOf,
            person_af::{PersonMotherAF, PersonParentsAF},
            person_at::PersonMotherAT,
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

    #[test]
    fn af_and_fold() {
        let lens = PersonParentsAF.then(ListOf);

        let mut parents = Fold::fold(&lens, Person::wojtek());
        assert_eq!(parents.next().unwrap().name, "Miroslawa");
        assert_eq!(parents.next().unwrap().name, "Zenon");
    }

    #[test]
    fn af_and_at() {
        let lens = PersonMotherAF.then(PersonMotherAT);

        let grandma = lens.preview(Person::wojtek());
        assert_eq!(grandma.unwrap().name, "Lidia");
    }

    #[test]
    fn af_and_traversal() {
        let lens = PersonParentsAF.then(Every);

        let mut parents = lens.fold(Person::wojtek());
        assert_eq!(parents.next().unwrap().name, "Miroslawa");
        assert_eq!(parents.next().unwrap().name, "Zenon");
    }
}
