use crate::prelude::*;

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
impl<T> GetterRef<AsGetter, Option<T>> for Unwrap {
    fn view_ref<'a>(&self, source: &'a Option<T>) -> &'a <Self as Getter<AsGetter, Option<T>>>::T {
        source.as_ref().unwrap()
    }

    fn impl_preview_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::T> {
        source.as_ref()
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

    #[test]
    fn as_ref() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.view_ref(&test), "Foo");
        assert_eq!(Unwrap.preview_ref(&test).unwrap(), "Foo");
        assert_eq!(Unwrap.fold_ref(&test).next().unwrap(), "Foo");

        // assert_eq!(Unwrap.view(&test), "Foo");
        // assert_eq!(Unwrap.preview(&test).unwrap(), "Foo");
        // assert_eq!(Unwrap.fold(&test).next().unwrap(), "Foo");
    }
}
