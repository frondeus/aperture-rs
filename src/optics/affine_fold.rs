use crate::prelude::*;

pub struct AsAffineMethod;
pub trait AffineFold<As, S>: Fold<As, S> {
    type T;
    fn preview(&self, source: S) -> Option<<Self as AffineFold<As, S>>::T>;
}

impl<A1, A2, L1, L2, S> AffineFold<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    L1::D: Iterator,
    L2: Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item> + Clone,
    <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,

    L1: AffineFold<A1, S>,
    L2: AffineFold<A2, L1::T> + Clone,
{
    type T = L2::T;

    fn preview(&self, source: S) -> Option<<Self as AffineFold<(A1, A2), S>>::T> {
        self.0.preview(source).and_then(|t| self.1.preview(t))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{
            lenses::{PersonMother, PersonName},
            Person,
        },
        prelude::*,
    };

    #[test]
    fn combinator() {
        let lens = PersonMother.then(PersonName);
        let moms_name = lens.preview(Person::olivier());
        assert_eq!(moms_name, Some("Anne".to_string()));
    }
}
