use crate::prelude::*;

pub struct FoldOf<F, TF>(pub F, pub TF);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold() {
        let test: Vec<u32> = vec![1, 2, 3];
        let folded = FoldOf(|x, y| x + y, || 0).fold(test);
        assert!(folded == 6);
    }
}
