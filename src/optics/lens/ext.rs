use super::*;

pub trait LensExt<'a, Src, Marker>: Sized {
    fn then<L2, GM, SM, TM>(self, l2: L2) -> LensAndLens<Self, L2>
    where
        L2: LensLike<'a, Src, GM, SM, TM>,
    {
        LensAndLens(self, l2)
    }

    fn into_lens(self) -> Lens<Self> {
        Lens(self)
    }
}

impl<'a, L, Src, GM, SM, TM> LensExt<'a, Src, (GM, SM, TM)> for L where
    L: LensLike<'a, Src, GM, SM, TM>
{
}
