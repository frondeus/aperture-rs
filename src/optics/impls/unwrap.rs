use crate::{optics::getter::AsGetter, prelude::*};

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
