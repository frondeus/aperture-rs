use crate::prelude::Getter;

#[derive(Default, Debug, Clone, Copy)]
pub struct Identity;

impl<R> Getter<R> for Identity {
    type T = R;

    fn view(&self, source: R) -> <Self as Getter<R>>::T {
        source
    }
}
