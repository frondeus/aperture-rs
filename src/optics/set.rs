pub trait SetLike<S, Marker> {
    type T;
    fn set<F>(&self, source: S, f: F) -> S
    where
        F: FnOnce(&mut Self::T);
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{Arg, Test},
        lazy::LazyExt,
    };

    use super::*;

    #[test]
    fn set() {
        let test = Test("Foo".into());

        let test = Test::mut_.set(test, |x| *x = "Bar".into());
        assert_eq!(test.0, "Bar");
        let test = Test::mut_arg
            .with_args((1,))
            .set(test, |x| *x = "Bar".into());
        assert_eq!(test.0, "Bar");
        let test = Test::mut_complex
            .lazy(|| (Arg,))
            .set(test, |x| *x = "Bar".into());
        assert_eq!(test.0, "Bar");
    }
}
