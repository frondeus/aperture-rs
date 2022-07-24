use crate::prelude::{And, Optics};

#[derive(Debug, Default)]
pub struct AsFold;
pub trait Fold<As, S> {
    type D;

    fn fold(&self, source: S) -> Self::D;
}

pub mod nested;

impl<A1, A2, L1, L2, S> Fold<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    L1::D: Iterator,
    L2: Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item> + Clone,
    L2::D: Iterator,
{
    type D = nested::NestedFold<A2, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        nested::NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

// impl<S, M> Fold<AsFold, S> for M where M: crate::method::Method<S, ()> {
//     type D;

//     fn fold(&self, source: S) -> Self::D {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            lenses::{PersonMother, PersonName},
            Person,
        },
        prelude::{fold_of::FoldOf, list_of::ListOf, Then},
    };

    #[test]
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        let mut iter = ListOf.fold(test);
        assert_eq!(iter.next().unwrap(), "Foo");

        let test: Vec<u32> = vec![1, 2, 3];
        let folded = FoldOf(|x, y| x + y, || 0).fold(test);
        assert!(folded == 6);
    }

    #[test]
    fn combinator() {
        let lens = PersonMother.then(PersonName);
        let mut iter = Fold::fold(&lens, Person::olivier());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("Anne".to_string()));
    }
}
