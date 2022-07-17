pub trait SetLike<'a, S, Marker> {
    type T: 'a;
    fn set<F>(&self, source: &'a mut S, f: F)
    where
        F: FnOnce(&'a mut Self::T);
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
        let mut test = Test("Foo".into());

        Test::mut_.set(&mut test, |x| *x = "Bar".into());
        assert_eq!(test.0, "Bar");
        Test::mut_arg
            .with_args((1,))
            .set(&mut test, |x| *x = "Bar".into());
        assert_eq!(test.0, "Bar");
        Test::mut_complex
            .lazy(|| (Arg,))
            .set(&mut test, |x| *x = "Bar".into());
        assert_eq!(test.0, "Bar");
    }
}
