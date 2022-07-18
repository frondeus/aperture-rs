pub struct AsSetter;
pub trait Setter<As, S> {
    type T;
    fn set<F>(&self, source: S, f: F) -> S
    where
        F: FnOnce(&mut Self::T);
}

#[cfg(test)]
pub fn assert_setter<Optic, S, As, M>(_o: Optic)
where
    Optic: Setter<As, S>,
{
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
