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
        // let inner = self.impl_preview(source).map(f);
        // self.impl_review(inner)
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

 )*};
}

impl_and!(
    AsPrism,
    // (AsPrism, AsIso),
);
