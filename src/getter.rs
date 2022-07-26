use crate::prelude::*;

pub struct AsGetter;
pub trait Getter<As, S> {
    type T;
    fn view(&self, source: S) -> <Self as Getter<As, S>>::T;

    fn impl_preview(&self, source: S) -> Option<Self::T> {
        Some(self.view(source))
    }
}
impl<S, X> Optics<AsGetter, S> for X where X: Getter<AsGetter, S> {}

impl<X, S> AffineFold<AsGetter, S> for X
where
    X: Getter<AsGetter, S>,
{
    type T = X::T;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}
impl<X, S> Fold<AsGetter, S> for X
where
    X: AffineFold<AsGetter, S>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l),)*); };
 (@ $(($l:ident, $r:ident),)*) => {$(
impl<L1, L2, S> Getter<AsGetter, S> for And<L1, L2, ($l, $r), (S, L1::T)>
where
    L1: Getter<$l, S>,
    L2: Getter<$r, L1::T>,
{
    type T = L2::T;

    fn view(&self, source: S) -> <Self as Getter<AsGetter, S>>::T {
        self.1.view(self.0.view(source))
    }
}
 )*};
}

impl_and!(
    AsGetter,
    (AsGetter, AsLens),
    // (AsGetter, AsRevPrism),
    // (AsGetter, AsIso),
    // (AsLens,   AsRevPrism),
);

// impl<A1, A2, L1, L2, S> Getter<(A1, A2), S> for And<L1, L2>
// where
//     L1: Getter<A1, S>,
//     L1::D: Iterator,
//     L2: Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
//     <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,
//     L2: Clone,
//     L2: Getter<A2, <L1 as Getter<A1, S>>::T>,
//     L1: AffineFold<A1, S>,
//     L2: AffineFold<A2, <L1 as AffineFold<A1, S>>::T>,
// {
//     type T = <L2 as Getter<A2, <L1 as Getter<A1, S>>::T>>::T;

//     fn view(&self, source: S) -> <Self as Getter<(A1, A2), S>>::T {
//         self.1.view(self.0.view(source))
//     }
// }

// impl<S, M, T> Getter<AsGetter, S> for M
// where
//     M: crate::method::Method<S, (), Output = T> + AffineFold<AsGetter, S>,
// {
//     type T = T;

//     fn view(&self, source: S) -> <Self as Getter<AsGetter, S>>::T {
//         self.mcall(source, ())
//     }
// }
// impl<S, M, T> AffineFold<AsGetter, S> for M
// where
//     M: crate::method::Method<S, (), Output = T> + Fold<AsGetter, S>,
// {
//     type T = T;

//     fn preview(&self, source: S) -> Option<<Self as AffineFold<AsGetter, S>>::T> {
//         Some(self.view(source))
//     }
// }
// impl<S, M, T> Fold<AsGetter, S> for M
// where
//     M: crate::method::Method<S, (), Output = T>,
// {
//     type D = std::option::IntoIter<T>;

//     fn fold(&self, source: S) -> Self::D {
//         self.preview(source).into_iter()
//     }
// }

//     #[test]
//     fn combinator() {
//         let lens = PersonMother.then(PersonName);
//         let moms_name = lens.view(Person::olivier());
//         assert_eq!(&moms_name, "Anne");
//     }

//     #[test]
//     fn method() {
//         let lens = PersonMother.then(Person::name);
//         let moms_name = lens.view(Person::olivier());
//         assert_eq!(&moms_name, "Anne");
//     }
