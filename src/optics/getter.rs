use crate::prelude::*;

pub struct AsGetter;
pub trait Getter<As, S>: AffineFold<As, S> {
    type T;
    fn view(&self, source: S) -> <Self as Getter<As, S>>::T;
}

impl<A1, A2, L1, L2, S> Getter<(A1, A2), S> for And<L1, L2>
where
    L1: Getter<A1, S>,
    L1::D: Iterator,
    L2: Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,
    L2: Clone,
    L2: Getter<A2, <L1 as Getter<A1, S>>::T>,
    L1: AffineFold<A1, S>,
    L2: AffineFold<A2, <L1 as AffineFold<A1, S>>::T>,
{
    type T = <L2 as Getter<A2, <L1 as Getter<A1, S>>::T>>::T;

    fn view(&self, source: S) -> <Self as Getter<(A1, A2), S>>::T {
        self.1.view(self.0.view(source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            lenses::{PersonMother, PersonName},
            Person,
        },
        prelude::{unwrap::Unwrap, Then},
    };

    #[test]
    fn view() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.view(test), "Foo");
    }

    #[test]
    fn as_affine_fold() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.preview(test), Some("Foo".to_string()));
    }

    #[test]
    fn as_fold() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Unwrap.fold(test).next(), Some("Foo".to_string()));
    }

    #[test]
    fn combinator() {
        let lens = PersonMother.then(PersonName);
        let moms_name = lens.view(Person::olivier());
        assert_eq!(&moms_name, "Anne");
    }
    // #[test]
    // fn as_affine_fold() {
    //     let test = Test("Foo".into());

    //     assert_eq!(
    //         Test::own_.as_affine_fold().preview(test),
    //         Some("Foo".to_string())
    //     );

    //     let test = Test("Foo".into());
    //     assert_eq!(
    //         Test::own_.as_affine_fold().fold(test).next(),
    //         Some("Foo".to_string())
    //     );
    // }
}
