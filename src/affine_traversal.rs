use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsAffineTraversal;
pub trait AffineTraversal<As, S> {
    type O;
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(Self::O) -> T,
    {
        self.impl_preview(source).map(f)
    }

    fn impl_preview(&self, source: S) -> Option<Self::O>;

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O;
}

impl<S, X> Optics<AsAffineTraversal, S> for X where X: AffineTraversal<AsAffineTraversal, S> {}
impl<X, S> AffineFold<AsAffineTraversal, S> for X
where
    X: AffineTraversal<AsAffineTraversal, S>,
{
    type T = X::O;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}

impl<X, S> Traversal<AsAffineTraversal, S> for X
where
    X: AffineTraversal<AsAffineTraversal, S>,
{
    type D = std::option::IntoIter<X::O>;

    fn impl_fold(&self, source: S) -> Self::D {
        self.impl_preview(source).into_iter()
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(<Self::D as Iterator>::Item) -> <Self::D as Iterator>::Item,
    {
        self.impl_set(source, f)
    }
}
impl<X, S> Setter<AsAffineTraversal, S> for X
where
    X: Traversal<AsAffineTraversal, S>,
{
    type O = <X::D as Iterator>::Item;

    type D = S;
    type T = <X::D as Iterator>::Item;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: Clone + FnMut(Self::O) -> Self::T + Clone,
    {
        self.impl_set(source, f)
    }
}
impl<X, S> Fold<AsAffineTraversal, S> for X
where
    X: AffineFold<AsAffineTraversal, S>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l),)*); };
 (@ $(($l:ident, $r:ident),)*) => {$(
impl<L1, L2, S> AffineTraversal<AsAffineTraversal, S>
    for And<L1, L2, ($l, $r), (S, L1::O)>
where
    L1: AffineTraversal<$l, S>,
    L2: AffineTraversal<$r, L1::O>,
{
    type O = L2::O;

    fn impl_preview(&self, source: S) -> Option<Self::O> {
        self.0
            .impl_preview(source)
            .and_then(|x| self.1.impl_preview(x))
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O,
    {
        self.0.set(source, |x| self.1.set(x, f.clone()))
    }
}
 )*};
}

impl_and!(
    AsAffineTraversal,
    // (AsAffineTraversal, AsLens),
    // (AsAffineTraversal, AsPrism),
    // (AsAffineTraversal, AsIso),
    // (AsLens, AsPrism),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
            person_af::PersonMotherAF,
            person_at::{PersonMotherAT, PersonParentsAT},
            person_folds::PersonParentsFold,
            person_setters::PersonNameSetter,
            *,
        },
    };

    #[test]
    fn at_and_at() {
        let lens = PersonMotherAT.then(PersonMotherAT);
        let grandma = lens.map_opt(Person::wojtek(), |n| n.name);
        assert_eq!(grandma.unwrap(), "Lidia");

        let lens = PersonMotherAT.then(PersonMotherAT);
        let wojtek = lens.set(Person::wojtek(), |mut n| {
            n.name = n.name.to_uppercase();
            n
        });
        assert_eq!(wojtek.parents[0].parents[0].name, "LIDIA");
    }

    #[test]
    fn at_and_setter() {
        let lens = PersonMotherAT.then(PersonNameSetter);
        let wojtek = lens.set(Person::wojtek(), |name| name.to_uppercase());
        assert_eq!(wojtek.parents[0].name, "MIROSLAWA");
    }

    #[test]
    fn at_and_fold() {
        let lens = PersonMotherAT.then(PersonParentsFold);
        let mut moms_parents = lens.fold(Person::wojtek());
        assert_eq!(moms_parents.next().unwrap().name, "Lidia");
        assert_eq!(moms_parents.next().unwrap().name, "Jerzy");
        assert_eq!(moms_parents.next(), None);
    }

    #[test]
    fn at_and_traversal() {
        let lens = PersonParentsAT.then(Every);
        let mut moms_parents = lens.traverse(Person::wojtek(), |x| x.name);
        assert_eq!(moms_parents.next().unwrap(), "Miroslawa");
        assert_eq!(moms_parents.next().unwrap(), "Zenon");
        assert_eq!(moms_parents.next(), None);
    }

    #[test]
    fn at_and_af() {
        let lens = PersonMotherAT.then(PersonMotherAF);
        let grandma = lens.preview(Person::wojtek());
        assert_eq!(grandma.unwrap().name, "Lidia");
    }
}
