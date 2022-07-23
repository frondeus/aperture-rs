// use crate::optics::{AffineFoldLike, GetLike, IsLens, LensLike, SetLike, Then};

use std::marker::PhantomData;

use crate::optics::{
    fold::nested::NestedFold,
    traversal::nested::NestedTraverse,
    AffineFold,
    AsLens,
    Fold,
    Getter,
    Lens,
    Setter,
    Then,
    Traversal,
};

pub struct LensAndLens<L1, L2>(pub L1, pub L2);

impl<S, L1, L2> Then<AsLens, S, L2> for L1
where
    L1: Setter<AsLens, S> + Fold<AsLens, S> + Traversal<AsLens, S>,
    <L1 as Fold<AsLens, S>>::D: Iterator,
{
    type Output = LensAndLens<L1, L2>;

    fn then(self, l2: L2) -> Self::Output {
        LensAndLens(self, l2)
    }
}

impl<L1, L2, S, T> Setter<AsLens, S> for LensAndLens<L1, L2>
where
    L1: Setter<AsLens, S, T = T, O = T, D = S>,
    L2: Setter<AsLens, T, D = T>,
{
    type O = L2::O;
    type T = L2::T;

    type D = S;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        self.0.set(source, |o| self.1.set(o, f.clone()))
    }
}

impl<L1, L2, S> Fold<AsLens, S> for LensAndLens<L1, L2>
where
    L1: Fold<AsLens, S>,
    L1::D: Iterator,
    L2: Fold<AsLens, <<L1 as Fold<AsLens, S>>::D as Iterator>::Item> + Clone,
    L2::D: Iterator,
{
    type D = NestedFold<AsLens, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

impl<L1, L2, S> AffineFold<AsLens, S> for LensAndLens<L1, L2>
where
    L1: Fold<AsLens, S>,
    L1::D: Iterator,
    L2: Fold<AsLens, <<L1 as Fold<AsLens, S>>::D as Iterator>::Item> + Clone,
    <L2 as Fold<AsLens, <<L1 as Fold<AsLens, S>>::D as Iterator>::Item>>::D: Iterator,

    L1: AffineFold<AsLens, S>,
    L2: AffineFold<AsLens, L1::T> + Clone,
{
    type T = L2::T;

    fn preview(&self, source: S) -> Option<<Self as AffineFold<AsLens, S>>::T> {
        self.0.preview(source).and_then(|t| self.1.preview(t))
    }
}

impl<L1, L2, S> Traversal<AsLens, S> for LensAndLens<L1, L2>
where
    L1: Fold<AsLens, S>,
    <L1 as Fold<AsLens, S>>::D: Iterator,
    L2: Clone + Fold<AsLens, <<L1 as Fold<AsLens, S>>::D as Iterator>::Item>,
    L2::D: Iterator,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(<Self::D as Iterator>::Item) -> T,
    {
        self.fold(source).map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{Person, Test},
        optics::{AffineFold, AffineTraversal, AsLens, Fold, Traversal},
    };

    fn olivier() -> Person {
        Person {
            age: 24,
            name: "Olivier".into(),
            parents: vec![
                Person {
                    age: 55,
                    name: "Anne".to_string(),
                    parents: vec![],
                },
                Person {
                    age: 56,
                    name: "Thierry".to_string(),
                    parents: vec![],
                },
            ],
        }
    }
    //     fn is_lens<'a, L: LensLike<'a, S, G, M>, S, G, M>(_l: L) {}
    #[test]
    fn as_view() {
        // let lens = PersonMother.then(PersonName);
        // let moms_name = lens.view(olivier());
        // assert_eq!(&moms_name, "Anne");
    }

    #[test]
    fn as_aff_fold() {
        let lens = PersonMother.then(PersonName);
        let moms_name = lens.preview(olivier());
        assert_eq!(moms_name, Some("Anne".to_string()));
    }

    #[test]
    fn as_setter() {
        let lens = PersonMother.then(PersonName);
        let new_olivier = lens.set(olivier(), |name| name.to_uppercase());
        assert_eq!(new_olivier.mother().name, "ANNE");

        // let mom = lens.view(olivier());
        // assert_eq!(&mom, "Anne");
    }

    #[test]
    fn as_fold() {
        let lens = PersonMother.then(PersonName);
        let mut iter = Fold::fold(&lens, olivier());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("Anne".to_string()));

        // let mom = lens.view(olivier());
        // assert_eq!(&mom, "Anne");
    }

    #[test]
    fn as_traversal() {
        let lens = PersonMother.then(PersonName);
        let mut iter = lens.traverse(olivier(), |name| name.to_uppercase());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("ANNE".to_string()));
    }

    #[derive(Clone)]
    struct PersonName;

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
        fn traverse<F, T>(&self, source: Person, f: F) -> std::iter::Map<Self::D, F>
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
    struct PersonMother;

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
        fn traverse<F, T>(&self, source: Person, f: F) -> std::iter::Map<Self::D, F>
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
    struct PersonParents;
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
        fn traverse<F, T>(&self, source: Person, f: F) -> std::iter::Map<Self::D, F>
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
}
