// mod tuple;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsLens;
pub trait Lens<S, As = AsLens> {
    type View;
    #[doc(hidden)]
    fn impl_view(&self, source: S) -> Self::View;

    #[doc(hidden)]
    fn impl_preview(&self, source: S) -> Option<Self::View> {
        Some(self.impl_view(source))
    }

    #[doc(hidden)]
    fn impl_set<F: Clone + FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S;
}

pub trait LensMut<S, As = AsLens>: Lens<S, As> {
    #[doc(hidden)]
    fn impl_set_mut<F: Clone + FnMut(&mut Self::View)>(&self, source: &mut S, f: F);
}

pub trait LensRef<S, As = AsLens>: LensMut<S, As> {
    #[doc(hidden)]
    fn impl_view_ref<'a>(&self, source: &'a S) -> &'a Self::View;

    #[doc(hidden)]
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::View> {
        Some(self.impl_view_ref(source))
    }
}

impl<S, X> Optics<S, AsLens> for X where X: Lens<S> {}

impl<X, S> Getter<S, AsLens> for X
where
    X: Lens<S>,
{
    type T = X::View;

    fn view(&self, source: S) -> <Self as Getter<S, AsLens>>::T {
        self.impl_view(source)
    }

    fn impl_preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}

impl<X, S> AffineFold<S, AsLens> for X
where
    X: Getter<S, AsLens>,
{
    type T = X::T;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}
impl<X, S> Fold<S, AsLens> for X
where
    X: AffineFold<S, AsLens>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}
impl<X, S> AffineTraversal<S, AsLens> for X
where
    X: Lens<S>,
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
impl<X, S> Traversal<S, AsLens> for X
where
    X: AffineTraversal<S, AsLens>,
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
impl<X, S> Setter<S, AsLens> for X
where
    X: Traversal<S, AsLens>,
{
    type O = <X::D as Iterator>::Item;

    fn set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O + Clone,
    {
        self.impl_set(source, f)
    }
}
impl<X, S> AffineTraversalMut<S, AsLens> for X
where
    X: LensMut<S>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::O),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> TraversalMut<S, AsLens> for X
where
    X: AffineTraversalMut<S, AsLens>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> SetterMut<S, AsLens> for X
where
    X: TraversalMut<S, AsLens>,
{
    fn set_mut<F>(&self, source: &mut S, f: F)
    where
        F: FnMut(&mut Self::O) + Clone,
    {
        self.impl_set_mut(source, f);
    }
}

impl<X, S> GetterRef<S, AsLens> for X
where
    X: LensRef<S>,
{
    fn view_ref<'a>(&self, source: &'a S) -> &'a <Self as Getter<S, AsLens>>::T {
        self.impl_view_ref(source)
    }

    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}

impl<X, S> AffineFoldRef<S, AsLens> for X
where
    X: GetterRef<S, AsLens>,
{
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> FoldRef<S, AsLens> for X
where
    X: AffineFoldRef<S, AsLens>,
    for<'a> X::T: 'a,
    for<'a> S: 'a,
{
    type Item<'a> = X::T;

    type DRef<'a> = std::option::IntoIter<&'a X::T>;

    fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.preview_ref(source).into_iter()
    }
}
impl<X, S> AffineTraversalRef<S, AsLens> for X
where
    X: LensRef<S>,
{
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::O> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> TraversalRef<S, AsLens> for X
where
    X: AffineTraversalRef<S, AsLens>,
{
    type DRef<'a> = std::option::IntoIter<&'a X::O>
    where
        <Self::D as Iterator>::Item: 'a,
        S: 'a;

    fn impl_fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.impl_preview_ref(source).into_iter()
    }
}

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l),)*); };
 (@ $(($l:ident, $r:ident),)*) => {$(
impl<L1, L2, S> Lens<S> for And<L1, L2, ($l, $r), (S, L1::View)>
where
    L1: Lens< S, $l>,
    L2: Lens< L1::View, $r>,
{
    type View = L2::View;

    fn impl_view(&self, source: S) -> Self::View {
        self.1.view(self.0.view(source))
    }

    fn impl_set<F: Clone + FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S {
        self.0.set(source, |p| self.1.set(p, f.clone()))
    }
}
impl<L1, L2, S> LensMut<S> for And<L1, L2, ($l, $r), (S, L1::View)>
where
    L1: LensMut< S, $l>,
    L2: LensMut< L1::View, $r>,
{
    fn impl_set_mut<F: Clone + FnMut(&mut Self::View)>(&self, source: &mut S, f: F) {
        self.0.impl_set_mut(source, |p|  self.1.impl_set_mut(p, f.clone())   )
    }
}
impl<L1, L2, S> LensRef< S> for And<L1, L2, ($l, $r), (S, L1::View)>
where
    L1: LensRef< S, $l>,
    L2: LensRef< L1::View, $r>,
    for<'a> L1::View: 'a
{
    #[doc(hidden)]
    fn impl_view_ref<'a>(&self, source: &'a S) -> &'a Self::View {
        self.1.impl_view_ref(self.0.impl_view_ref(source))
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
        data::{Person, PersonLensesExt as _},
        prelude::person_lenses::PersonLensesExt,
        std::Every,
    };

    #[test]
    fn derived() {
        let age = Person::mother.then_age().view(Person::olivier());
        assert_eq!(age, 55);
    }

    #[test]
    fn lens_and_lens() {
        let telescope = Person::mother.then_name();

        let name = telescope.view(Person::olivier());
        assert_eq!(name, "Anne");

        let olivier = telescope.set(Person::olivier(), |name| name.to_uppercase());
        assert_eq!(olivier.parents[0].name, "ANNE");
    }

    #[test]
    fn lens_and_lens_mut() {
        let lens = Person::mother.then_name();

        let mut olivier = Person::olivier();

        lens.set_mut(&mut olivier, |name| *name = name.to_uppercase());
        assert_eq!(&olivier.parents[0].name, "ANNE");
    }

    #[test]
    fn lens_and_lens_ref() {
        let lens = Person::mother.then_name();

        let wojtek = Person::wojtek();

        let name = lens.view_ref(&wojtek);
        assert_eq!(name, "Miroslawa");
    }

    #[test]
    fn telescope() {
        let telescope = Person::mother.then_parents().then(Every).then(Person::name);

        let wojtek = Person::wojtek();

        let mut iter = telescope.fold_ref(&wojtek);
        assert_eq!(iter.next().unwrap(), "Lidia");
    }

    #[test]
    fn and_is_valid_lens() {
        let lens = Person::mother.then_mother().then_name();

        let name = lens.view(Person::wojtek());
        assert_eq!(name, "Lidia");

        let wojtek = lens.set(Person::wojtek(), |name| name.to_uppercase());
        assert_eq!(wojtek.parents[0].name, "Miroslawa");
        assert_eq!(wojtek.parents[0].parents[0].name, "LIDIA");
    }

    #[test]
    fn and_is_valid_lens_ref() {
        let lens = Person::mother.then_mother().then_name();

        let mut wojtek = Person::wojtek();
        let name = lens.view_ref(&wojtek);
        assert_eq!(name, "Lidia");

        lens.set_mut(&mut wojtek, |name| *name = name.to_uppercase());
        assert_eq!(wojtek.parents[0].name, "Miroslawa");
        assert_eq!(wojtek.parents[0].parents[0].name, "LIDIA");
    }

    #[test]
    fn as_getter() {
        let mom = Person::mother.view(Person::olivier());
        assert_eq!(&mom.name, "Anne");

        let parents = Person::parents.view(Person::olivier());
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents[1].name, "Thierry");
    }

    #[test]
    fn as_affine_fold() {
        let mom: Option<Person> = Person::mother.preview(Person::olivier());
        assert_eq!(mom.unwrap().name, "Anne");

        let parents = Person::parents.preview(Person::olivier());
        let parents = parents.unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents[1].name, "Thierry");
    }

    #[test]
    fn as_fold() {
        let mut mom = Person::mother.fold(Person::olivier());
        assert_eq!(mom.next().unwrap().name, "Anne");

        let mut parents = Person::parents.fold(Person::olivier());
        let parents = parents.next().unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents[1].name, "Thierry");
    }

    #[test]
    fn as_affine_traversal() {
        let mom = Person::mother.map_opt(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        assert_eq!(mom.unwrap().name, "Jocelyn");

        let parents = Person::parents.map_opt(Person::olivier(), |mut parents| {
            parents.pop();
            parents
        });
        let parents = parents.unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents.len(), 1);
    }

    #[test]
    fn as_traversal() {
        let mut mom = Person::mother.traverse(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        assert_eq!(mom.next().unwrap().name, "Jocelyn");

        let mut parents = Person::parents.traverse(Person::olivier(), |mut parents| {
            parents.pop();
            parents
        });
        let parents = parents.next().unwrap();
        assert_eq!(parents[0].name, "Anne");
        assert_eq!(parents.len(), 1);
    }

    #[test]
    fn as_setter() {
        let new_olivier = Person::mother.set(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        assert_eq!(new_olivier.parents[0].name, "Jocelyn");

        let new_olivier = Person::parents.set(Person::olivier(), |mut parents| {
            parents.pop();
            parents
        });
        assert_eq!(new_olivier.parents[0].name, "Anne");
        assert_eq!(new_olivier.parents.len(), 1);
    }
}
