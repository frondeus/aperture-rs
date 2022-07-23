// 0 degree - Basic blocks
mod setter;
pub use setter::*;

mod review;
pub use review::*;

mod fold;
pub use fold::*;

// 1st degree
mod affine_fold;
pub use affine_fold::*;

mod traversal;
pub use traversal::*;

// 2nd degree
mod affine_traversal; // known as optional type
pub use affine_traversal::*;

mod getter;
pub use getter::*;

// 3rd degree - Complex
mod lens;
pub use lens::*;
// mod rev_lens;
// mod prism;
// mod rev_prism;

// 4th degree
// mod iso;

// Combinators
mod then;
pub use then::*;
mod then_impl;
