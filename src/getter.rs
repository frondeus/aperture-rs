use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsGetter;
pub trait Getter<S, As = AsGetter> {
    type T;
    fn view(&self, source: S) -> <Self as Getter<S, As>>::T;

    #[doc(hidden)]
    fn impl_preview(&self, source: S) -> Option<Self::T> {
        Some(self.view(source))
    }
}
pub trait GetterRef<S, As = AsGetter>: Getter<S, As> {
    fn view_ref<'a>(&self, source: &'a S) -> &'a <Self as Getter<S, As>>::T;

    #[doc(hidden)]
    fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        Some(self.view_ref(source))
    }
}

impl<S, X> Optics<S, AsGetter> for X where X: Getter<S> {}

impl<X, S> AffineFold<S, AsGetter> for X
where
    X: Getter<S>,
{
    type T = X::T;

    fn preview(&self, source: S) -> Option<Self::T> {
        self.impl_preview(source)
    }
}
impl<X, S> Fold<S, AsGetter> for X
where
    X: AffineFold<S, AsGetter>,
{
    type D = std::option::IntoIter<X::T>;

    fn fold(&self, source: S) -> Self::D {
        self.preview(source).into_iter()
    }
}

impl<X, S> AffineFoldRef<S, AsGetter> for X
where
    X: GetterRef<S>,
{
    fn preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
        self.impl_preview_ref(source)
    }
}
impl<X, S> FoldRef<S, AsGetter> for X
where
    X: AffineFoldRef<S, AsGetter>,
    for<'a> X::T: 'a,
    for<'a> S: 'a,
{
    type Item<'a> = X::T;

    type DRef<'a> = std::option::IntoIter<&'a X::T>;

    fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.preview_ref(source).into_iter()
    }
}

// impl<X, S, T> GetterRef<AsGetter, S> for X
// where
//     X: for<'b> Getter<AsGetter, &'b S, T = &'b T>,
//     X: Getter<AsGetter, S, T = T>,
// {
//     fn view_ref<'a>(&self, source: &'a S) -> &'a T {
//         self.view(source)
//     }

//     #[doc(hidden)]
//     fn impl_preview_ref<'a>(&self, source: &'a S) -> Option<&'a Self::T> {
//         self.impl_preview(source)
//     }
// }

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l),)*); };
 (@ $(($l:ident, $r:ident),)*) => {$(
impl<L1, L2, S> Getter<S> for And<L1, L2, ($l, $r), (S, L1::T)>
where
    L1: Getter< S, $l>,
    L2: Getter< L1::T, $r>,
{
    type T = L2::T;

    fn view(&self, source: S) -> <Self as Getter<S>>::T {
        self.1.view(self.0.view(source))
    }
}
impl<L1, L2, S> GetterRef<S> for And<L1, L2, ($l, $r), (S, L1::T)>
where
    L1: GetterRef< S, $l>,
    L2: GetterRef< L1::T, $r>,
    for<'a> L1::T: 'a
{
    fn view_ref<'a>(&self, source: &'a S) -> &'a Self::T {
        self.1.view_ref(self.0.view_ref(source))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::unwrap::Unwrap;

    #[test]
    fn getter_and_getter() {
        let test = Some(Some(4));

        assert_eq!(Unwrap.then(Unwrap).view(test), 4);
    }

    #[test]
    fn getter_and_getter_ref() {
        let test = Some(Some(4));

        assert_eq!(Unwrap.then(Unwrap).view_ref(&test), &4);
    }
}

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
