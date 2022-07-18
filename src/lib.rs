#![feature(unboxed_closures, fn_traits, type_alias_impl_trait)]

mod sealed {
    pub trait Sealed<S, A> {}
}
pub mod identity;
pub mod lazy;
pub mod method;
pub mod optics;

#[cfg(test)]
mod data;
