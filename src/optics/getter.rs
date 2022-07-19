use super::{AffineFold, Fold};
use crate::method::Method;

pub struct AsGetterMethod;
pub trait Getter<As, S> {
    type T;
    fn view(&self, source: S) -> <Self as Getter<As, S>>::T;
    fn as_affine_fold(self) -> AffineFoldImpl<Self>
    where
        Self: Sized,
    {
        AffineFoldImpl(self)
    }
}

pub struct AffineFoldImpl<Optics>(Optics);

impl<S, M, T> Getter<AsGetterMethod, S> for M
where
    M: Method<S, (), Output = T>,
{
    type T = T;

    fn view(&self, source: S) -> <Self as Getter<AsGetterMethod, S>>::T {
        self.mcall(source, ())
    }
}

impl<S, M, T> AffineFold<AsGetterMethod, S> for AffineFoldImpl<M>
where
    M: Method<S, (), Output = T>,
    Self: Fold<AsGetterMethod, S, T = T>,
{
    fn preview(&self, source: S) -> Option<T> {
        Some(self.0.mcall(source, ()))
    }
}

impl<S, M, T> Fold<AsGetterMethod, S> for AffineFoldImpl<M>
where
    M: Method<S, (), Output = T>,
{
    type T = T;

    type FoldIter = std::option::IntoIter<T>;

    fn fold(&self, source: S) -> Self::FoldIter {
        Some(self.0.mcall(source, ())).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{Arg, Test},
        lazy::LazyExt,
    };

    #[test]
    fn view() {
        let test = Test("Foo".into());

        assert_eq!(Test::ref_.view(&test), "Foo");
        assert_eq!(Test::ref_arg.with_args((1,)).view(&test), "Foo");
        // assert_eq!(Test::ref_complex.with_args((Arg,)).view_once(&test), "Foo");
        assert_eq!(Test::ref_complex.lazy(|| (Arg,)).view(&test), "Foo");
    }

    #[test]
    fn as_affine_fold() {
        let test = Test("Foo".into());

        assert_eq!(
            Test::own_.as_affine_fold().preview(test),
            Some("Foo".to_string())
        );

        let test = Test("Foo".into());
        assert_eq!(
            Test::own_.as_affine_fold().fold(test).next(),
            Some("Foo".to_string())
        );
    }
}
