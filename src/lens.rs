// mod tuple;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsLens;
pub trait Lens<As, S> {
    type View;
    fn impl_view(&self, source: S) -> Self::View;
    fn impl_set<F: Clone + FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S;
}
impl<S, X> Optics<AsLens, S> for X where X: Lens<AsLens, S> {}

impl<X, S> Getter<AsLens, S> for X
where
    X: Lens<AsLens, S>,
{
    type T = X::View;

    fn view(&self, source: S) -> <Self as Getter<AsLens, S>>::T {
        self.impl_view(source)
    }
}

impl<X, S> AffineFold<AsLens, S> for X
where
    X: Getter<AsLens, S>,
{
    type T = X::T;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}
impl<X, S> Fold<AsLens, S> for X
where
    X: AffineFold<AsLens, S>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}
impl<X, S> AffineTraversal<AsLens, S> for X
where
    X: Lens<AsLens, S>,
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
impl<X, S> Traversal<AsLens, S> for X
where
    X: AffineTraversal<AsLens, S>,
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
impl<X, S> Setter<AsLens, S> for X
where
    X: Traversal<AsLens, S>,
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

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l),)*); };
 (@ $(($l:ident, $r:ident),)*) => {$(
impl<L1, L2, S> Lens<AsLens, S> for And<L1, L2, ($l, $r), (S, L1::View)>
where
    L1: Lens<$l, S>,
    L2: Lens<$r, L1::View>,
{
    type View = L2::View;

    fn impl_view(&self, source: S) -> Self::View {
        self.1.view(self.0.view(source))
    }

    fn impl_set<F: Clone + FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S {
        self.0.set(source, |p| self.1.set(p, f.clone()))
    }
}
 )*};
}

impl_and!(
    AsLens,
    // (AsLens, AsIso),
);
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
    fn and_is_valid_lens() {
        let lens = PersonMother.then(PersonMother).then(PersonName);

        let name = lens.view(Person::wojtek());
        assert_eq!(name, "Lidia");

        let wojtek = lens.set(Person::wojtek(), |name| name.to_uppercase());
        assert_eq!(wojtek.parents[0].name, "Miroslawa");
        assert_eq!(wojtek.parents[0].parents[0].name, "LIDIA");
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
