use super::*;

pub trait IntoLens<'a, Src, Marker>: Sized {
    // fn into_lens(self) -> Lens<Self> {
    //     Lens(self)
    // }
}

impl<'a, L, Src, GM, SM, TM> IntoLens<'a, Src, (GM, SM, TM)> for L where
    L: LensLike<'a, Src, GM, SM, TM>
{
}
