use crate::{optics::fold::AsFold, prelude::*};

pub struct ListOf;
impl<S> Optics<AsFold, S> for ListOf {}

impl<S> Fold<AsFold, S> for ListOf
where
    S: IntoIterator,
{
    type D = S::IntoIter;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter()
    }
}
