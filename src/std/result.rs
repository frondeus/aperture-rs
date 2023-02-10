use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct _Ok;

impl<T, E> Prism<Result<T, E>> for _Ok {
    type Variant = T;

    fn impl_preview(&self, source: Result<T, E>) -> Option<Self::Variant> {
        source.ok()
    }

    fn impl_review(&self, variant: Self::Variant) -> Result<T, E> {
        Result::Ok(variant)
    }

    fn impl_set<F>(&self, source: Result<T, E>, f: F) -> Result<T, E>
    where
        F: Clone + FnMut(Self::Variant) -> Self::Variant,
    {
        source.map(f)
    }
}

impl<T, E> PrismMut<Result<T, E>> for _Ok {
    fn impl_set_mut<F>(&self, source: &mut Result<T, E>, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
        let _ = source.as_mut().map(f);
    }
}

impl<T, E> PrismRef<Result<T, E>> for _Ok {
    fn impl_preview_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::Variant> {
        source.as_ref().ok()
    }
}

#[derive(Clone, Copy)]
pub struct _Err;

impl<T, E> Prism<Result<T, E>> for _Err {
    type Variant = E;

    fn impl_preview(&self, source: Result<T, E>) -> Option<Self::Variant> {
        source.err()
    }

    fn impl_review(&self, variant: Self::Variant) -> Result<T, E> {
        Result::Err(variant)
    }

    fn impl_set<F>(&self, source: Result<T, E>, f: F) -> Result<T, E>
    where
        F: Clone + FnMut(Self::Variant) -> Self::Variant,
    {
        source.map_err(f)
    }
}

impl<T, E> PrismMut<Result<T, E>> for _Err {
    fn impl_set_mut<F>(&self, source: &mut Result<T, E>, f: F)
    where
        F: Clone + FnMut(&mut Self::Variant),
    {
        let _ = source.as_mut().map_err(f);
    }
}

impl<T, E> PrismRef<Result<T, E>> for _Err {
    fn impl_preview_ref<'a>(&self, source: &'a Result<T, E>) -> Option<&'a Self::Variant> {
        source.as_ref().err()
    }
}

#[allow(non_upper_case_globals)]
pub trait ResultExt {
    const _Ok: _Ok = _Ok;
    const _Err: _Err = _Err;
}
impl<T, E> ResultExt for Result<T, E> {}
