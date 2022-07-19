// 0 degree - Basic blocks
mod fold;
mod review;
mod setter;

// 1st degree
mod affine_fold;
mod traversal;

// 2nd degree
mod affine_traversal; // known as optional type

mod getter;

// 3rd degree - Complex
// mod rev_lens;
// mod prism;
mod lens;
// mod rev_prism;

// 4th degree
// mod iso;

// Combinators
// mod then;
// mod then_impl;

pub use affine_fold::*;
pub use affine_traversal::*;
pub use fold::*;
pub use getter::*;
pub use lens::*;
pub use review::*;
pub use setter::*;
pub use traversal::*;
// pub use get::*;
// pub use lens::*;
// pub use optional::*;
// pub use prism::*;
// pub use then::*;
