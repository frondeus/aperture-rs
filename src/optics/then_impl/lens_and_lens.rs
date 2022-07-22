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

pub struct LensAndLens<L1, L2, Mark>(pub L1, pub L2, PhantomData<Mark>);

impl<S, L1, L2, T1> Then<T1, S, L2> for L1
where
    L1: Setter<AsLens, S, T1> + Fold<AsLens, S>,
{
    type Output = LensAndLens<L1, L2, T1>;

    fn then(self, l2: L2) -> Self::Output {
        LensAndLens(self, l2, PhantomData)
    }
}

impl<L1, L2, S, T1, T2> Setter<AsLens, S, T2> for LensAndLens<L1, L2, T1>
where
    L1: Setter<AsLens, S, T1, O = T1, D = S>,
    L2: Setter<AsLens, T1, T2, O = T2, D = T1>,
{
    type O = T2;

    type D = S;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> T2 + Clone,
    {
        self.0.set(source, |o| self.1.set(o, f.clone()))
    }
}

impl<L1, L2, S, D1, D2, T1, T2, M> Fold<AsLens, S> for LensAndLens<L1, L2, M>
where
    L1: Fold<AsLens, S, D = D1>,
    L2: Fold<AsLens, T1, D = D2> + Clone,
    D1: Iterator<Item = T1>,
    D2: Iterator<Item = T2>,
{
    type D = NestedFold<AsLens, D1, L2>;

    fn fold(&self, source: S) -> Self::D {
        NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

impl<L1, L2, S, D1, D2, T1, T2, M> AffineFold<AsLens, S> for LensAndLens<L1, L2, M>
where
    L1: AffineFold<AsLens, S>,
    L2: AffineFold<AsLens, L1::T> + Clone,
    L1: Fold<AsLens, S, D = D1>,
    L2: Fold<AsLens, T1, D = D2> + Clone,
    D1: Iterator<Item = T1>,
    D2: Iterator<Item = T2>,
{
    type T = L2::T;

    fn preview(&self, source: S) -> Option<<Self as AffineFold<AsLens, S>>::T> {
        self.0.preview(source).and_then(|t| self.1.preview(t))
    }
}

// impl<L1, L2, S, T, F, M> Traversal<AsLens, S, T, F> for LensAndLens<L1, L2, M>
// where
//     L2: Traversal<AsLens, S, T, F>,
//     L1: Fold<AsLens, S>,
//     L2: Fold<AsLens, T> + Clone,
//     L1::D: Iterator,
//     <L2 as Fold<AsLens, T>>::D: Iterator<Item = T>,
//     F: FnMut(<L2 as Traversal<AsLens, S, T, F>>::O) -> T,
// {
//     type O = <L2 as Traversal<AsLens, S, T, F>>::O;

//     type D = NestedTraverse<AsLens, L1::D, L2, F>;

//     fn traverse(&self, source: S, f: F) -> <Self as Traversal<AsLens, S, T, F>>::D {
//         NestedTraverse::new(self.0.fold(source), self.1.clone(), f)
//     }
// }

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
        // let lens = PersonMother.then(PersonName);
        // // let mut iter = lens.traversal(olivier(), |name| name.to_uppercase());
        // let mums_name = iter.next();
        // assert_eq!(mums_name, Some("ANNE".to_string()));

        // let mom = lens.view(olivier());
        // assert_eq!(&mom, "Anne");
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
    impl<T, F> AffineTraversal<AsLens, Person, T, F> for PersonName
    where
        F: FnMut(String) -> T,
    {
        fn map_opt(&self, source: Person, f: F) -> Option<T> {
            Some(source.name).map(f)
        }
    }
    impl<T, F> Traversal<AsLens, Person, T, F> for PersonName
    where
        F: FnMut(String) -> T,
    {
        type O = String;

        type D = std::option::IntoIter<T>;

        fn traverse(&self, source: Person, f: F) -> <Self as Traversal<AsLens, Person, T, F>>::D {
            Some(source.name).map(f).into_iter()
        }
    }
    impl Setter<AsLens, Person, String> for PersonName {
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
    impl<T, F> AffineTraversal<AsLens, Person, T, F> for PersonMother
    where
        F: FnMut(Person) -> T,
    {
        fn map_opt(&self, source: Person, f: F) -> Option<T> {
            source.parents.into_iter().take(1).map(f).next()
        }
    }
    impl<T, F> Traversal<AsLens, Person, T, F> for PersonMother
    where
        F: FnMut(Person) -> T,
    {
        type O = Person;

        type D = std::iter::Map<std::iter::Take<std::vec::IntoIter<Person>>, F>;

        fn traverse(&self, source: Person, f: F) -> <Self as Traversal<AsLens, Person, T, F>>::D {
            source.parents.into_iter().take(1).map(f)
        }
    }
    impl Setter<AsLens, Person, Person> for PersonMother {
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
    impl<T, F> AffineTraversal<AsLens, Person, T, F> for PersonParents
    where
        F: FnMut(Vec<Person>) -> T,
    {
        fn map_opt(&self, source: Person, f: F) -> Option<T> {
            Some(source.parents).map(f)
        }
    }
    impl<T, F> Traversal<AsLens, Person, T, F> for PersonParents
    where
        F: FnMut(Vec<Person>) -> T,
    {
        type O = Vec<Person>;

        type D = std::option::IntoIter<T>;

        fn traverse(&self, source: Person, f: F) -> <Self as Traversal<AsLens, Person, T, F>>::D {
            Some(source.parents).map(f).into_iter()
        }
    }
    impl Setter<AsLens, Person, Vec<Person>> for PersonParents {
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
