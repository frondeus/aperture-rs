use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct _Some;

impl<T> Prism<Option<T>> for _Some {
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
impl<T> PrismMut<Option<T>> for _Some {
    fn impl_set_mut<F>(&self, source: &mut Option<T>, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
        source.as_mut().map(f);
    }
}
impl<T> PrismRef<Option<T>> for _Some {
    fn impl_preview_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::Variant> {
        source.as_ref()
    }
}

#[derive(Clone, Copy)]
pub struct _None;

impl<T> Prism<Option<T>> for _None {
    type Variant = ();

    fn impl_preview(&self, source: Option<T>) -> Option<Self::Variant> {
        match source {
            Some(_) => None,
            None => Some(()),
        }
    }

    fn impl_review(&self, _variant: Self::Variant) -> Option<T> {
        Option::None
    }

    fn impl_set<F>(&self, source: Option<T>, _f: F) -> Option<T>
    where
        F: Clone + FnMut(Self::Variant) -> Self::Variant,
    {
        source
    }
}
impl<T> PrismMut<Option<T>> for _None {
    fn impl_set_mut<F>(&self, _source: &mut Option<T>, _f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
    }
}
impl<T> PrismRef<Option<T>> for _None {
    fn impl_preview_ref<'a>(&self, source: &'a Option<T>) -> Option<&'a Self::Variant> {
        match source {
            Some(_) => None,
            None => Some(&()),
        }
    }
}

#[allow(non_upper_case_globals)]
pub trait OptionExt {
    const _Some: _Some = _Some;
    const _None: _None = _None;
}
impl<T> OptionExt for Option<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Person;

    #[test]
    fn as_review() {
        assert_eq!(Option::<u32>::_Some.review(4), Some(4));
    }

    #[test]
    fn as_at() {
        assert_eq!(Option::<u32>::_Some.map_opt(Some(4), |x| x + 1), Some(5));
        assert_eq!(
            Option::<u32>::_Some.map_opt(Option::<u32>::None, |x| x + 1),
            None
        );
    }

    #[test]
    fn as_af() {
        assert_eq!(Option::<u32>::_Some.preview(Some(4)), Some(4));
        assert_eq!(Option::<u32>::_Some.preview(Option::<u32>::None), None);
    }

    #[test]
    fn as_traversal() {
        assert_eq!(
            Option::<u32>::_Some.traverse(Some(4), |x| x + 1).next(),
            Some(5)
        );
        assert_eq!(
            Option::<u32>::_Some
                .traverse(Option::<u32>::None, |x| x + 1)
                .next(),
            None
        );
    }

    #[test]
    fn as_fold() {
        assert_eq!(Option::<u32>::_Some.fold(Some(4)).next(), Some(4));
        assert_eq!(Option::<u32>::_Some.fold(Option::<u32>::None).next(), None);
    }

    #[test]
    fn as_setter() {
        assert_eq!(Option::<u32>::_Some.set(Some(4), |x| x + 1), Some(5));
        assert_eq!(
            Option::<u32>::_Some.set(Option::<u32>::None, |x| x + 1),
            None
        );
    }

    #[test]
    fn as_setter_mut() {
        let mut x = Some(4);
        let mut y: Option<u32> = None;
        Option::<u32>::_Some.set_mut(&mut x, |x| *x += 1);
        Option::<u32>::_Some.set_mut(&mut y, |x| *x += 1);
        assert_eq!(x, Some(5));
        assert_eq!(y, None);
    }

    #[test]
    fn as_fold_ref() {
        let x = Some(4);
        let y: Option<u32> = None;

        assert_eq!(Option::<u32>::_Some.fold_ref(&x).next(), Some(&4));
        assert_eq!(Option::<u32>::_Some.fold_ref(&y).next(), None);
    }

    #[test]
    fn as_af_ref() {
        let x = Some(4);
        let y: Option<u32> = None;

        assert_eq!(Option::<u32>::_Some.preview_ref(&x), Some(&4));
        assert_eq!(Option::<u32>::_Some.preview_ref(&y), None);
    }

    #[test]
    fn in_telescope() {
        let telescope = Person::boss.then(Option::<Box<Person>>::_None);

        let wojtek = Person::wojtek();
        let has_boss = telescope.preview(wojtek);
        assert_eq!(Some(()), has_boss);

        let telescope = Person::boss.then(Option::<Box<Person>>::_Some);

        let wojtek = Person::boss.set(Person::wojtek(), |_| Some(Box::new(Person::olivier())));

        let has_boss = telescope.preview(wojtek);
        assert_eq!(Some(Box::new(Person::olivier())), has_boss);
    }
}
