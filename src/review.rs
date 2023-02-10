// use crate::method::Method;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsReview;
pub trait Review<S, As = AsReview> {
    type T;
    fn review(&self, t: Self::T) -> S;
}
impl<S, X> Optics<S, AsReview> for X where X: Review<S> {}

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l),)*); };
 (@ $(($l:ident, $r:ident),)*) => {$(
impl<L1, L2, S, S2> Review< S2> for And<L1, L2, ($l, $r), (S, S2)>
where
    L1: Review< S,$l>,
    L2: Review< S2,$r, T = S>,
{
    type T = L1::T;

    fn review(&self, t: Self::T) -> S2 {
        self.1.review(self.0.review(t))
    }
}
 )*};
}

impl_and!(
    AsReview,
    // (AsReview, AsRevLens),
    (AsReview, AsPrism),
    // (AsReview, AsIso),
    // (AsRevLens, Prism),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::some_review::SomeR;
    // use crate::{
    //     data::{Arg, Test},
    //     lazy::LazyExt,
    // };
    #[test]
    fn review_and_review() {
        let lens = SomeR.then(SomeR);
        let res = lens.review(5);
        assert_eq!(res, Option::Some(Option::Some(5)));
    }

    // #[test]
    // fn review() {
    //     let test: Option<String> = Some("Foo".into());

    //     assert_eq!(Option::unwrap.review(test), "Foo");

    //     let test: Option<String> = Some("Foo".into());

    //     assert_eq!(
    //         Option::unwrap_or_else
    //             .with_args((|| String::from("Bar"),))
    //             .review(test),
    //         "Foo"
    //     );

    //     let test: Option<String> = Some("Foo".into());

    //     assert_eq!(
    //         Option::unwrap_or.lazy(|| ("Bar".into(),)).review(test),
    //         "Foo"
    //     );

    //     let test = Test("Foo".into());

    //     assert_eq!(Test::own_complex.lazy(|| (Arg,)).review(test), "Foo");

    //     let test: String = "Foo".into();

    //     assert_eq!(Option::Some.review(test), Some("Foo".into()));
    // }
}
