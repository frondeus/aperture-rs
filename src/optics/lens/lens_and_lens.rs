use super::*;

pub struct LensAndLens<L1, L2>(pub L1, pub L2);

impl<'a, Src, M1, M2, L1, L2> GetLike<'a, Src, (IsLens, M1, M2)> for LensAndLens<L1, L2>
where
    L1: GetLike<'a, Src, M1>,
    L2: GetLike<'a, L1::T, M2>,
    Src: 'a,
{
    type T = L2::T;

    fn view(&self, source: &'a Src) -> &'a Self::T {
        self.1.view(self.0.view(source))
    }
}

impl<'a, Src, M1, M2, L1, L2> SetLike<'a, Src, (IsLens, M1, M2)> for LensAndLens<L1, L2>
where
    L1: SetLike<'a, Src, M1>,
    L2: SetLike<'a, L1::T, M2>,
    Src: 'a,
{
    type T = L2::T;

    fn set<F>(&self, source: &'a mut Src, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        self.0.set(source, |t| self.1.set(t, f))
    }
}

impl<'a, Src, M1, M2, L1, L2> TraversalLike<'a, Src, (IsLens, M1, M2)> for LensAndLens<L1, L2>
where
    L1: TraversalLike<'a, Src, M1>,
    L2: TraversalLike<'a, L1::T, M2>,
    Src: 'a,
{
    type T = L2::T;

    fn preview(&self, source: &'a Src) -> Option<&'a Self::T> {
        self.1.preview(self.0.preview(source)?)
    }
}
