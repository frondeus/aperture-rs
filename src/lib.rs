// #![feature(unboxed_closures, fn_traits, type_alias_impl_trait)]
// #![allow(unused_imports)]
#![deny(clippy::all)]
// #![deny(clippy::pedantic)]

mod sealed {
    pub trait Sealed<S, A> {}
}
pub mod identity;
pub mod lazy;
pub mod method;
pub mod optics;

#[cfg(test)]
mod data;

pub mod prelude {
    pub use crate::optics::prelude::*;
}
