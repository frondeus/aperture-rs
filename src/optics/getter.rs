use super::{AffineFold, Fold};
use crate::method::Method;

pub struct AsGetter;
pub trait Getter<As, S>: AffineFold<As, S> {
    type T;
    fn view(&self, source: S) -> <Self as Getter<As, S>>::T;
}

pub struct Unwrap;
impl<T> Getter<AsGetter, Option<T>> for Unwrap {
    type T = T;

    fn view(&self, source: Option<T>) -> <Self as Getter<AsGetter, Option<T>>>::T {
        source.unwrap()
    }
}
impl<T> Fold<AsGetter, Option<T>> for Unwrap {
    type D = std::option::IntoIter<T>;

    fn fold(&self, source: Option<T>) -> Self::D {
        source.into_iter()
    }
}
impl<T> AffineFold<AsGetter, Option<T>> for Unwrap {
    type T = T;
    fn preview(&self, source: Option<T>) -> Option<T> {
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.view(test), "Foo");
    }

    #[test]
    fn as_affine_fold() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.preview(test), Some("Foo".to_string()));
    }

    #[test]
    fn as_fold() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.fold(test).next(), Some("Foo".to_string()));
    }
    // #[test]
    // fn as_affine_fold() {
    //     let test = Test("Foo".into());

    //     assert_eq!(
    //         Test::own_.as_affine_fold().preview(test),
    //         Some("Foo".to_string())
    //     );

    //     let test = Test("Foo".into());
    //     assert_eq!(
    //         Test::own_.as_affine_fold().fold(test).next(),
    //         Some("Foo".to_string())
    //     );
    // }
}
