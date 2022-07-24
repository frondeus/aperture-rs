use std::fmt::Debug;

use crate::prelude::And;
pub trait Optics<As, S> {
    fn is(&self) -> As
    where
        As: Debug + Default,
    {
        As::default()
    }
}
