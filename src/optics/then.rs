pub trait Then<As, S, L2>: Sized {
    type Output;

    fn then(self, l2: L2) -> Self::Output;
}
