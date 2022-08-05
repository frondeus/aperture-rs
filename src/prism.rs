use crate::prelude::*;

pub struct AsPrism;
pub trait Prism<As, S> {
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

pub trait PrismMut<As, S>: Prism<As, S> {
    #[doc(hidden)]
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant);
}
pub trait PrismRef<As, S>: PrismMut<As, S> {
    #[doc(hidden)]
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::Variant>;
}

impl<S, X> Optics<AsPrism, S> for X where X: Prism<AsPrism, S> {}

impl<X, S> Review<AsPrism, S> for X
where
    X: Prism<AsPrism, S>,
{
    type T = X::Variant;

    fn review(&self, t: Self::T) -> S {
        self.impl_review(t)
    }
}

impl<X, S> AffineTraversal<AsPrism, S> for X
where
    X: Prism<AsPrism, S>,
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

impl<X, S> AffineFold<AsPrism, S> for X
where
    X: AffineTraversal<AsPrism, S>,
{
    type T = X::O;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}

impl<X, S> Fold<AsPrism, S> for X
where
    X: AffineFold<AsPrism, S>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

impl<X, S> Traversal<AsPrism, S> for X
where
    X: AffineTraversal<AsPrism, S>,
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

impl<X, S> Setter<AsPrism, S> for X
where
    X: Traversal<AsPrism, S>,
{
    type O = <X::D as Iterator>::Item;

    fn set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O + Clone,
    {
        self.impl_set(source, f)
    }
}
impl<X, S> AffineTraversalMut<AsPrism, S> for X
where
    X: PrismMut<AsPrism, S>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::O),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> TraversalMut<AsPrism, S> for X
where
    X: AffineTraversalMut<AsPrism, S>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item),
    {
        self.impl_set_mut(source, f);
    }
}
impl<X, S> SetterMut<AsPrism, S> for X
where
    X: TraversalMut<AsPrism, S>,
{
    fn set_mut<F>(&self, source: &mut S, f: F)
    where
        F: FnMut(&mut Self::O) + Clone,
    {
        self.impl_set_mut(source, f);
    }
}

impl<X, S> AffineTraversalRef<AsPrism, S> for X
where
    X: PrismRef<AsPrism, S>,
{
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::O> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> AffineFoldRef<AsPrism, S> for X
where
    X: AffineTraversalRef<AsPrism, S>,
{
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> TraversalRef<AsPrism, S> for X
where
    X: AffineTraversalRef<AsPrism, S>,
    for<'a> X::O: 'a,
    for<'a> S: 'a,
{
    type Item<'a> = X::O;

    type DRef<'a> = std::option::IntoIter<&'a X::O>;

    fn impl_fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.impl_preview_ref(source).into_iter()
    }
}
impl<X, S> FoldRef<AsPrism, S> for X
where
    X: AffineFoldRef<AsPrism, S>,
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
impl <L1, L2, S> Prism<AsPrism, S> for And<L1, L2, ($l, $r), (S, L1::Variant)>
where
L1: Prism<$l, S>,
L2: Prism<$r, L1::Variant> {
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
impl <L1, L2, S> PrismMut<AsPrism, S> for And<L1, L2, ($l, $r), (S, L1::Variant)>
where
L1: PrismMut<$l, S>,
L2: PrismMut<$r, L1::Variant> {
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
        self.0.impl_set_mut(source, |x| self.1.impl_set_mut(x, f.clone()));
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
    use crate::prelude::some::Some;

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

        prism.set_mut(&mut src_some, |x| *x = *x + 1);
        assert_eq!(src_some, Option::Some(Option::Some(5)));
    }
}
