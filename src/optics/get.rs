pub trait GetLike<'a, S, Marker> {
    type T: 'a;
    fn view(&self, source: &'a S) -> &'a Self::T;
}

pub trait GetLikeOnce<'a, S> {
    type T: 'a;
    fn view_once(self, source: &'a S) -> &'a Self::T;
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{Arg, Test},
        lazy::LazyExt,
    };

    use super::*;

    #[test]
    fn view() {
        let test = Test("Foo".into());

        assert_eq!(Test::ref_.view_once(&test), "Foo");
        assert_eq!(Test::ref_arg.with_args((1,)).view_once(&test), "Foo");
        assert_eq!(Test::ref_complex.with_args((Arg,)).view_once(&test), "Foo");
        assert_eq!(Test::ref_complex.lazy(|| (Arg,)).view(&test), "Foo");
    }
}
