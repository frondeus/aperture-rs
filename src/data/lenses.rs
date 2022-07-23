use super::Person;
use crate::{optics::lens::AsLens, prelude::*};

#[derive(Clone)]
pub struct PersonName;

impl Getter<AsLens, Person> for PersonName {
    type T = String;

    fn view(&self, source: Person) -> <Self as Getter<AsLens, Person>>::T {
        source.name
    }
}
impl AffineFold<AsLens, Person> for PersonName {
    type T = String;

    fn preview(&self, source: Person) -> Option<<Self as AffineFold<AsLens, Person>>::T> {
        Some(source.name)
    }
}
impl Fold<AsLens, Person> for PersonName {
    type D = std::option::IntoIter<String>;

    fn fold(&self, source: Person) -> Self::D {
        Some(source.name).into_iter()
    }
}
impl AffineTraversal<AsLens, Person> for PersonName {
    fn map_opt<T, F>(&self, source: Person, f: F) -> Option<T>
    where
        F: FnOnce(String) -> T,
    {
        Some(source.name).map(f)
    }
}
impl Traversal<AsLens, Person> for PersonName {
    fn traverse<F, T>(
        &self,
        source: Person,
        f: F,
    ) -> std::iter::Map<<Self as Fold<AsLens, Person>>::D, F>
    where
        F: FnMut(String) -> T,
    {
        Some(source.name).into_iter().map(f)
    }
}
impl Setter<AsLens, Person> for PersonName {
    type T = String;
    type O = String;

    type D = Person;

    fn set<F>(&self, mut source: Person, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> String,
    {
        source.name = f(source.name);
        source
    }
}
pub struct PersonMother;

impl Getter<AsLens, Person> for PersonMother {
    type T = Person;

    fn view(&self, source: Person) -> <Self as Getter<AsLens, Person>>::T {
        source.parents.into_iter().next().unwrap()
    }
}
impl AffineFold<AsLens, Person> for PersonMother {
    type T = Person;

    fn preview(&self, source: Person) -> Option<<Self as AffineFold<AsLens, Person>>::T> {
        source.parents.into_iter().next()
    }
}
impl Fold<AsLens, Person> for PersonMother {
    type D = std::iter::Take<std::vec::IntoIter<Person>>;

    fn fold(&self, source: Person) -> Self::D {
        source.parents.into_iter().take(1)
    }
}
impl AffineTraversal<AsLens, Person> for PersonMother {
    fn map_opt<T, F>(&self, source: Person, f: F) -> Option<T>
    where
        F: FnOnce(Person) -> T,
    {
        source.parents.into_iter().take(1).next().map(f)
    }
}
impl Traversal<AsLens, Person> for PersonMother {
    fn traverse<F, T>(
        &self,
        source: Person,
        f: F,
    ) -> std::iter::Map<<Self as Fold<AsLens, Person>>::D, F>
    where
        F: FnMut(Person) -> T,
    {
        source.parents.into_iter().take(1).map(f)
    }
}
impl Setter<AsLens, Person> for PersonMother {
    type T = Person;
    type O = Person;

    type D = Person;

    fn set<F>(&self, mut source: Person, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Person,
    {
        let mut iter = source.parents.into_iter();
        let new_mom = iter.next().map(f);
        source.parents = new_mom.into_iter().chain(iter).collect();
        source
    }
}
pub struct PersonParents;
impl Getter<AsLens, Person> for PersonParents {
    type T = Vec<Person>;

    fn view(&self, source: Person) -> <Self as Getter<AsLens, Person>>::T {
        source.parents
    }
}
impl AffineFold<AsLens, Person> for PersonParents {
    type T = Vec<Person>;

    fn preview(&self, source: Person) -> Option<<Self as AffineFold<AsLens, Person>>::T> {
        Some(source.parents)
    }
}
impl Fold<AsLens, Person> for PersonParents {
    type D = std::option::IntoIter<Vec<Person>>;

    fn fold(&self, source: Person) -> Self::D {
        Some(source.parents).into_iter()
    }
}
impl AffineTraversal<AsLens, Person> for PersonParents {
    fn map_opt<T, F>(&self, source: Person, f: F) -> Option<T>
    where
        F: FnOnce(Vec<Person>) -> T,
    {
        Some(source.parents).map(f)
    }
}
impl Traversal<AsLens, Person> for PersonParents {
    fn traverse<F, T>(
        &self,
        source: Person,
        f: F,
    ) -> std::iter::Map<<Self as Fold<AsLens, Person>>::D, F>
    where
        F: FnMut(Vec<Person>) -> T,
    {
        Some(source.parents).into_iter().map(f)
    }
}
impl Setter<AsLens, Person> for PersonParents {
    type T = Vec<Person>;
    type O = Vec<Person>;

    type D = Person;

    fn set<F>(&self, mut source: Person, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Vec<Person>,
    {
        source.parents = f(source.parents);
        source
    }
}
