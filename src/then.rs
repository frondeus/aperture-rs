use std::marker::PhantomData;

use crate::Optics;

pub trait Then<As, L2, S>: Sized {
    fn then(self, l2: L2) -> And<Self, L2, As, S> {
        And(self, l2, PhantomData, PhantomData)
    }
}
#[derive(Default, Debug)]
pub struct And<L1, L2, As, S>(pub L1, pub L2, PhantomData<As>, PhantomData<S>);

impl<L1, L2, S1, S2, A1, A2> Then<(A1, A2), L2, (S1, S2)> for L1
where
    L1: Optics<A1, S1>,
    L2: Optics<A2, S2>,
    A1: Default + std::fmt::Debug,
    A2: Default + std::fmt::Debug,
{
}
