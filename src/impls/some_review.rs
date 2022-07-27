use crate::prelude::*;

#[derive(Clone)]
pub struct SomeR;

impl<T> Review<AsReview, Option<T>> for SomeR {
    type T = T;

    fn review(&self, t: Self::T) -> Option<T> {
        Option::Some(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_review() {
        assert_eq!(SomeR.review(4), Option::Some(4));
    }
}
