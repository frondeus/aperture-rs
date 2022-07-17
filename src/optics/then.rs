pub trait Then<'a, Src, L2, Marker>: Sized {
    type Output;

    fn then(self, l2: L2) -> Self::Output;
}
