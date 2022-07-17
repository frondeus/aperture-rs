use super::{ReviewLike, TraversalLike};

pub trait PrismLike<'a, S, TM>: ReviewLike<'a, S> + TraversalLike<'a, S, TM> {}
