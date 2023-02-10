use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsPrism;
pub trait Prism<S, As = AsPrism> {
    type Variant;
    #[doc(hidden)]
    fn impl_preview(&self, source: S) -> Option<Self::Variant>;
    #[doc(hidden)]
    fn impl_review(&self, variant: Self::Variant) -> S;
    #[doc(hidden)]
    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::Variant) -> Self::Variant;
}

pub trait PrismMut<S, As = AsPrism>: Prism<S, As> {
    #[doc(hidden)]
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant);
}
pub trait PrismRef<S, As = AsPrism>: PrismMut<S, As> {
    #[doc(hidden)]
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::Variant>;
}

impl<S, X> Optics<S, AsPrism> for X where X: Prism<S, AsPrism> {}

impl<X, S> Review<S, AsPrism> for X
where
    X: Prism<S>,
{
    type T = X::Variant;

    fn review(&self, t: Self::T) -> S {
        self.impl_review(t)
    }
}

impl<X, S> AffineTraversal<S, AsPrism> for X
where
    X: Prism<S>,
{
    type O = X::Variant;

    fn impl_preview(&self, source: S) -> Option<Self::O> {
        Prism::impl_preview(self, source)
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O,
    {
        Prism::impl_set(self, source, f)
    }
}

impl<X, S> AffineFold<S, AsPrism> for X
where
    X: AffineTraversal<S, AsPrism>,
{
    type T = X::O;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}

impl<X, S> Fold<S, AsPrism> for X
where
    X: AffineFold<S, AsPrism>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

impl<X, S> Traversal<S, AsPrism> for X
where
    X: AffineTraversal<S, AsPrism>,
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

impl<X, S> Setter<S, AsPrism> for X
where
    X: Traversal<S, AsPrism>,
{
    type O = <X::D as Iterator>::Item;

    fn set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O + Clone,
    {
        self.impl_set(source, f)
    }
}
impl<X, S> AffineTraversalMut<S, AsPrism> for X
where
    X: PrismMut<S>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::O),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> TraversalMut<S, AsPrism> for X
where
    X: AffineTraversalMut<S, AsPrism>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> SetterMut<S, AsPrism> for X
where
    X: TraversalMut<S, AsPrism>,
{
    fn set_mut<F>(&self, source: &mut S, f: F)
    where
        F: FnMut(&mut Self::O) + Clone,
    {
        self.impl_set_mut(source, f);
    }
}

impl<X, S> AffineTraversalRef<S, AsPrism> for X
where
    X: PrismRef<S>,
{
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::O> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> AffineFoldRef<S, AsPrism> for X
where
    X: AffineTraversalRef<S, AsPrism>,
{
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> TraversalRef<S, AsPrism> for X
where
    X: AffineTraversalRef<S, AsPrism>,
    for<'a> X::O: 'a,
    for<'a> S: 'a,
{
    // type Item<'a> = X::O;

    type DRef<'a> = std::option::IntoIter<&'a X::O>;

    fn impl_fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.impl_preview_ref(source).into_iter()
    }
}
impl<X, S> FoldRef<S, AsPrism> for X
where
    X: AffineFoldRef<S, AsPrism>,
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
impl <L1, L2, S> Prism<S> for And<L1, L2, ($l, $r), (S, L1::Variant)>
where
L1: Prism< S, $l>,
L2: Prism< L1::Variant, $r> {
    type Variant = L2::Variant;

    fn impl_preview(&self, source: S) -> Option<Self::Variant> {
        self.0
            .impl_preview(source)
            .and_then(|x| self.1.impl_preview(x))
    }

    fn impl_review(&self, variant: Self::Variant) -> S {
        self.0.impl_review(self.1.impl_review(variant))
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::Variant) -> Self::Variant,
    {
        self.0.impl_set(source, |x| self.1.impl_set(x, f.clone()))
    }
}
impl <L1, L2, S> PrismMut<S> for And<L1, L2, ($l, $r), (S, L1::Variant)>
where
L1: PrismMut<S, $l>,
L2: PrismMut<L1::Variant, $r> {
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
        self.0.impl_set_mut(source, |x| self.1.impl_set_mut(x, f.clone()));
    }
}
impl <L1, L2, S> PrismRef<S> for And<L1, L2, ($l, $r), (S, L1::Variant)>
where
L1: PrismRef< S, $l>,
L2: PrismRef< L1::Variant, $r>,
for<'a> L1::Variant: 'a
{
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::Variant> {
        self.0.impl_preview_ref(source)
                .and_then(|x| self.1.impl_preview_ref(x))
    }
}

 )*};
}

impl_and!(
    AsPrism,
    // (AsPrism, AsIso),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data::TestEnum, prelude::some::Some};

    #[test]
    fn prism_and_prism() {
        let prism = Some.then(Some);

        let src_some: Option<Option<u32>> = prism.review(4);
        assert_eq!(src_some, Option::Some(Option::Some(4)));

        let deep_some = prism.preview(src_some);
        assert_eq!(deep_some, Option::Some(4));
    }

    #[test]
    fn prism_and_prism_mut() {
        let prism = Some.then(Some);

        let mut src_some: Option<Option<u32>> = prism.review(4);

        prism.set_mut(&mut src_some, |x| *x += 1);
        assert_eq!(src_some, Option::Some(Option::Some(5)));
    }

    #[test]
    fn prism_and_prism_ref() {
        let prism = Some.then(Some);

        let src_some: Option<Option<u32>> = prism.review(4);
        assert_eq!(src_some, Option::Some(Option::Some(4)));

        let deep_some = prism.preview_ref(&src_some);
        assert_eq!(deep_some, Option::Some(&4));
    }

    #[test]
    fn derived_prism_preview() {
        let prism = TestEnum::v1;
        let prism_b = TestEnum::v2;

        let a = TestEnum::V1("Foo".into());
        let b = TestEnum::V2;

        let previewed = prism.preview(a);
        assert_eq!(previewed, Option::Some("Foo".to_string()));

        let previewed = prism.preview(b);
        assert_eq!(previewed, None);

        let a = TestEnum::V1("Foo".into());
        let b = TestEnum::V2;

        let previewed = prism_b.preview(a);
        assert_eq!(previewed, Option::None);

        let previewed = prism_b.preview(b);
        assert_eq!(previewed, Option::Some(()));
    }

    #[test]
    fn derived_prism_review() {
        let prism = TestEnum::v1;
        let prism_b = TestEnum::v2;

        let a: String = "Foo".into();
        let b = ();

        let reviewed = prism.review(a.clone());
        assert_eq!(reviewed, TestEnum::V1(a));

        let reviewed = prism_b.review(b);
        assert_eq!(reviewed, TestEnum::V2);
    }

    #[test]
    fn derived_prism_set() {
        let prism = TestEnum::v1;

        let a = TestEnum::V1("Foo".into());
        let b = TestEnum::V2;

        let new_a = prism.set(a, |x| x.to_uppercase());
        assert_eq!(new_a, TestEnum::V1("FOO".to_string()));

        let new_b = prism.set(b, |x| x.to_uppercase());
        assert_eq!(new_b, TestEnum::V2);
    }
}
