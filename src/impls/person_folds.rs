use crate::{
    data::Person,
    prelude::{AsFold, Fold},
};

#[derive(Clone)]
pub struct PersonParentsFold;
impl Fold<AsFold, Person> for PersonParentsFold {
    type D = std::vec::IntoIter<Person>;

    fn fold(&self, source: Person) -> Self::D {
        source.parents.into_iter()
    }
}

#[derive(Clone)]
pub struct PersonGrandParentsFold;
impl Fold<AsFold, Person> for PersonGrandParentsFold {
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
