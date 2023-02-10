use crate::prelude::*;

#[derive(Clone)]
pub struct Some;

impl<T> Prism<Option<T>> for Some {
    type Variant = T;

    fn impl_preview(&self, source: Option<T>) -> Option<Self::Variant> {
        source
    }

    fn impl_review(&self, variant: Self::Variant) -> Option<T> {
        Option::Some(variant)
    }

    fn impl_set<F>(&self, source: Option<T>, f: F) -> Option<T>
    where
        F: Clone + FnMut(Self::Variant) -> Self::Variant,
    {
        source.map(f)
    }
}
impl<T> PrismMut<Option<T>> for Some {
    fn impl_set_mut<F>(&self, source: &mut Option<T>, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
        source.as_mut().map(f);
    }
}
impl<T> PrismRef<Option<T>> for Some {
    fn impl_preview_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::Variant> {
        source.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_review() {
        assert_eq!(Some.review(4), Option::Some(4));
    }

    #[test]
    fn as_at() {
        assert_eq!(Some.map_opt(Option::Some(4), |x| x + 1), Option::Some(5));
        assert_eq!(Some.map_opt(Option::<u32>::None, |x| x + 1), None);
    }

    #[test]
    fn as_af() {
        assert_eq!(Some.preview(Option::Some(4)), Option::Some(4));
        assert_eq!(Some.preview(Option::<u32>::None), None);
    }

    #[test]
    fn as_traversal() {
        assert_eq!(
            Some.traverse(Option::Some(4), |x| x + 1).next(),
            Option::Some(5)
        );
        assert_eq!(Some.traverse(Option::<u32>::None, |x| x + 1).next(), None);
    }

    #[test]
    fn as_fold() {
        assert_eq!(Some.fold(Option::Some(4)).next(), Option::Some(4));
        assert_eq!(Some.fold(Option::<u32>::None).next(), None);
    }

    #[test]
    fn as_setter() {
        assert_eq!(Some.set(Option::Some(4), |x| x + 1), Option::Some(5));
        assert_eq!(Some.set(Option::<u32>::None, |x| x + 1), None);
    }

    #[test]
    fn as_setter_mut() {
        let mut x = Option::Some(4);
        let mut y: Option<u32> = None;
        Some.set_mut(&mut x, |x| *x += 1);
        Some.set_mut(&mut y, |x| *x += 1);
        assert_eq!(x, Option::Some(5));
        assert_eq!(y, None);
    }

    #[test]
    fn as_fold_ref() {
        let x = Option::Some(4);
        let y: Option<u32> = None;

        assert_eq!(Some.fold_ref(&x).next(), Option::Some(&4));
        assert_eq!(Some.fold_ref(&y).next(), None);
    }

    #[test]
    fn as_af_ref() {
        let x = Option::Some(4);
        let y: Option<u32> = None;

        assert_eq!(Some.preview_ref(&x), Option::Some(&4));
        assert_eq!(Some.preview_ref(&y), None);
    }
}
