use crate::{data::Person, prelude::*};

#[derive(Clone)]
pub struct PersonParentsFold;
impl Fold<Person> for PersonParentsFold {
    type D = std::vec::IntoIter<Person>;

    fn fold(&self, source: Person) -> Self::D {
        source.parents.into_iter()
    }
}
// #[cfg(feature = "gat")]
impl FoldRef<Person> for PersonParentsFold {
    type Item<'a> = Person;

    type DRef<'a> = std::slice::Iter<'a, Person>;

    fn fold_ref<'a>(&self, source: &'a Person) -> Self::DRef<'a> {
        source.parents.iter()
    }
}

#[derive(Clone, Default)]
pub struct PersonParentsFoldRef<'a>(std::marker::PhantomData<&'a ()>);
impl<'a> Fold<Person> for PersonParentsFoldRef<'a> {
    type D = std::vec::IntoIter<Person>;

    fn fold(&self, source: Person) -> Self::D {
        source.parents.into_iter()
    }
}
// #[cfg(not(feature = "gat"))]
// impl<'a> FoldRef<'a, Person> for PersonParentsFoldRef<'a> {
//     type DRef = std::slice::Iter<'a, Person>;

//     fn fold_ref(&self, source: &'a Person) -> Self::DRef {
//         source.parents.iter()
//     }
// }

#[derive(Clone)]
pub struct PersonGrandParentsFold;
impl Fold<Person> for PersonGrandParentsFold {
    type D = std::vec::IntoIter<Vec<Person>>;

    fn fold(&self, source: Person) -> Self::D {
        source
            .parents
            .into_iter()
            .map(|x| x.parents)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

// impl
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_fold() {
        let mut iter = PersonParentsFoldRef::default().fold(Person::wojtek());
        assert_eq!(iter.next().unwrap().name, "Miroslawa");
        assert_eq!(iter.next().unwrap().name, "Zenon");
        assert_eq!(iter.next(), None);

        let mut iter = PersonParentsFold.fold(Person::wojtek());
        assert_eq!(iter.next().unwrap().name, "Miroslawa");
        assert_eq!(iter.next().unwrap().name, "Zenon");
        assert_eq!(iter.next(), None);
    }

    // #[cfg(not(feature = "gat"))]
    // #[test]
    // fn as_fold_ref() {
    //     let wojtek = Person::wojtek();
    //     let mut iter = PersonParentsFoldRef::default().fold_ref(&wojtek);
    //     assert_eq!(iter.next().unwrap().name, "Miroslawa");
    //     assert_eq!(iter.next().unwrap().name, "Zenon");
    //     assert_eq!(iter.next(), None);
    // }

    // #[cfg(feature = "gat")]
    #[test]
    fn as_fold_ref_gat() {
        let wojtek = Person::wojtek();
        let mut iter = PersonParentsFold.fold_ref(&wojtek);
        assert_eq!(iter.next().unwrap().name, "Miroslawa");
        assert_eq!(iter.next().unwrap().name, "Zenon");
        assert_eq!(iter.next(), None);
    }
}
