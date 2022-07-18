// 0 degree - Basic blocks
mod fold;
mod review;
mod set;

// 1st degree
mod affine_fold;
mod traversal;

// 2nd degree
mod affine_traversal; // known as optional type

// mod get;

// 3rd degree - Complex
// mod rev_lens;
// mod prism;
// mod lens;
// mod rev_prism;

// 4th degree
// mod iso;

// Impl
mod impls;

// Combinators
// mod then;
// mod then_impl;

pub use review::*;
pub use set::*;

pub use affine_fold::*;
pub use traversal::*;

pub use affine_traversal::*;
// pub use get::*;
// pub use lens::*;
// pub use optional::*;
// pub use prism::*;
// pub use then::*;
