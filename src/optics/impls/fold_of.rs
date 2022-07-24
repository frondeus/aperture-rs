use crate::{optics::fold::AsFold, prelude::*};

pub struct FoldOf<F, TF>(pub F, pub TF);
impl<S, F, TF> Optics<AsFold, S> for FoldOf<F, TF> {}

impl<S, F, T, TF> Fold<AsFold, S> for FoldOf<F, TF>
where
    S: IntoIterator<Item = T>,
    F: FnMut(T, T) -> T,
    TF: Fn() -> T,
    F: Copy,
{
    type D = T;

    fn fold(&self, source: S) -> Self::D {
        let t = (self.1)();
        source.into_iter().fold(t, self.0)
    }
}
