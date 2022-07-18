use crate::optics::{GetLike, LensLike, PrismLike, Then};

pub struct LensAndPrism<L, P>(pub L, pub P);

impl<'a, L, P, Src, GM1, SM1, TM2> Then<'a, Src, P, (GM1, SM1, TM2)> for L
where
    L: LensLike<'a, Src, GM1, SM1> + GetLike<'a, Src, GM1>,
    P: PrismLike<'a, <L as GetLike<'a, Src, GM1>>::T, TM2>,
{
    type Output = LensAndPrism<L, P>; // Opt

    fn then(self, l2: P) -> Self::Output {
        LensAndPrism(self, l2)
    }
}
