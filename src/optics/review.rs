use crate::method::Method;

#[derive(Debug, Default)]
pub struct AsReview;
pub trait Review<As, S> {
    type T;
    fn review(&self, t: Self::T) -> S;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{Arg, Test},
        lazy::LazyExt,
    };

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
