pub trait Then<L2>: Sized {
    fn then(self, l2: L2) -> And<Self, L2>;
}
pub struct And<L1, L2>(pub L1, pub L2);

impl<L1, L2> Then<L2> for L1
where
    L1: Sized, // where
               //     L1: Setter<AsLens, S> + Fold<AsLens, S> + Traversal<AsLens, S> + Getter<AsLens, S>,
               //     <L1 as Fold<AsLens, S>>::D: Iterator,
{
    fn then(self, l2: L2) -> And<Self, L2> {
        And(self, l2)
    }
}
