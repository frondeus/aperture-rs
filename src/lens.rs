// mod tuple;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsLens;
pub trait Lens<As, S> {
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

pub trait LensMut<As, S>: Lens<As, S> {
    #[doc(hidden)]
    fn impl_set_mut<F: Clone + FnMut(&mut Self::View)>(&self, source: &mut S, f: F);
}

pub trait LensRef<As, S>: LensMut<As, S> {
    #[doc(hidden)]
    fn impl_view_ref<'a>(&self, source: &'a S) -> &'a Self::View;

    #[doc(hidden)]
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::View> {
        Some(self.impl_view_ref(source))
    }
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

    fn impl_preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
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

    fn set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O + Clone,
    {
        self.impl_set(source, f)
    }
}
impl<X, S> AffineTraversalMut<AsLens, S> for X
where
    X: LensMut<AsLens, S>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::O),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> TraversalMut<AsLens, S> for X
where
    X: AffineTraversalMut<AsLens, S>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> SetterMut<AsLens, S> for X
where
    X: TraversalMut<AsLens, S>,
{
    fn set_mut<F>(&self, source: &mut S, f: F)
    where
        F: FnMut(&mut Self::O) + Clone,
    {
        self.impl_set_mut(source, f);
    }
}

impl<X, S> GetterRef<AsLens, S> for X
where
    X: LensRef<AsLens, S>,
{
    fn view_ref<'a>(&self, source: &'a S) -> &'a <Self as Getter<AsLens, S>>::T {
        self.impl_view_ref(source)
    }

    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}

impl<X, S> AffineFoldRef<AsLens, S> for X
where
    X: GetterRef<AsLens, S>,
{
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> FoldRef<AsLens, S> for X
where
    X: AffineFoldRef<AsLens, S>,
    for<'a> X::T: 'a,
    for<'a> S: 'a,
{
    type Item<'a> = X::T;

    type DRef<'a> = std::option::IntoIter<&'a X::T>;

    fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.preview_ref(source).into_iter()
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
impl<L1, L2, S> LensMut<AsLens, S> for And<L1, L2, ($l, $r), (S, L1::View)>
where
    L1: LensMut<$l, S>,
    L2: LensMut<$r, L1::View>,
{
    fn impl_set_mut<F: Clone + FnMut(&mut Self::View)>(&self, source: &mut S, f: F) {
        self.0.impl_set_mut(source, |p|  self.1.impl_set_mut(p, f.clone())   )
    }
}
impl<L1, L2, S> LensRef<AsLens, S> for And<L1, L2, ($l, $r), (S, L1::View)>
where
    L1: LensRef<$l, S>,
    L2: LensRef<$r, L1::View>,
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
    };

    #[test]
    fn derived() {
        let age = Person::mother.then_age().view(Person::olivier());
        assert_eq!(age, 55);
    }

    #[test]
    fn lens_and_lens() {
        // let lens = Person::mother.then(Person::name);
        let lens = Person::mother.then_name();

        let name = lens.view(Person::olivier());
        assert_eq!(name, "Anne");

        let olivier = lens.set(Person::olivier(), |name| name.to_uppercase());
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
