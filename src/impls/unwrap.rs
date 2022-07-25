use crate::{optics::getter::AsGetter, prelude::*};

#[derive(Default, Debug, Clone)]
pub struct Unwrap;
impl<T> Getter<AsGetter, Option<T>> for Unwrap {
    type T = T;

    fn view(&self, source: Option<T>) -> <Self as Getter<AsGetter, Option<T>>>::T {
        source.unwrap()
    }

    fn impl_preview(&self, source: Option<T>) -> Option<Self::T> {
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
}
