// use crate::optics::{AffineFoldLike, GetLike, IsLens, LensLike, SetLike, Then};

use std::marker::PhantomData;

use crate::optics::{
    fold::nested::NestedFold,
    AffineFold,
    AffineTraversal,
    And,
    AsLens,
    Fold,
    Getter,
    Lens,
    Setter,
    Then,
    Traversal,
};

impl<A1, A2, L1, L2, S, T> Setter<(A1, A2), S> for And<L1, L2>
where
    L1: Setter<A1, S, T = T, O = T, D = S>,
    L2: Setter<A2, T, D = T>,
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

impl<A1, A2, L1, L2, S> Fold<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    L1::D: Iterator,
    L2: Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item> + Clone,
    L2::D: Iterator,
{
    type D = NestedFold<A2, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        NestedFold::new(self.0.fold(source), self.1.clone())
    }
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

impl<A1, A2, L1, L2, S> Traversal<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    <L1 as Fold<A1, S>>::D: Iterator,
    L2: Clone + Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    L2::D: Iterator,
    L1: Traversal<A1, S>,
    L2: Traversal<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(<Self::D as Iterator>::Item) -> T,
    {
        self.fold(source).map(f)
    }
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

impl<A1, A2, L1, L2, S, Item> AffineTraversal<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    <L1 as Fold<A1, S>>::D: Iterator,
    L2: Clone + Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    <L2 as Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>>::D: Iterator,
    <L2 as Fold<A2, <L1 as AffineFold<A1, S>>::T>>::D: Iterator<Item = Item>,
    L1: Traversal<A1, S>,
    L2: Traversal<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item>,
    L1: AffineFold<A1, S>,
    L2: AffineFold<A2, L1::T> + Clone,
    L1: AffineTraversal<A1, S>,
    L2: AffineTraversal<A2, L1::T>,
    Self: Fold<(A1, A2), S>,
    Self::D: Iterator<Item = Item>,
{
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(<Self::D as Iterator>::Item) -> T,
    {
        self.0.preview(source).and_then(|t| self.1.map_opt(t, f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            lenses::{PersonMother, PersonName, PersonParents},
            Person,
            Test,
        },
        optics::{AffineFold, AffineTraversal, AsLens, Every, Filtered, Fold, Traversal},
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
    fn as_setter() {
        let lens: And<PersonMother, PersonName> = PersonMother.then(PersonName);
        let new_olivier = lens.set(olivier(), |name| name.to_uppercase());
        assert_eq!(new_olivier.mother().name, "ANNE");
    }

    #[test]
    fn as_fold() {
        let lens = PersonMother.then(PersonName);
        let mut iter = Fold::fold(&lens, olivier());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("Anne".to_string()));
    }

    #[test]
    fn as_aff_fold() {
        let lens = PersonMother.then(PersonName);
        let moms_name = lens.preview(olivier());
        assert_eq!(moms_name, Some("Anne".to_string()));
    }

    #[test]
    fn as_traversal() {
        let lens = PersonMother.then(PersonName);
        let mut iter = lens.traverse(olivier(), |name| name.to_uppercase());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("ANNE".to_string()));
    }

    #[test]
    fn as_view() {
        let lens = PersonMother.then(PersonName);
        let moms_name = lens.view(olivier());
        assert_eq!(&moms_name, "Anne");
    }

    #[test]
    fn as_aff_traversal() {
        let lens = PersonMother.then(PersonName);
        let mums_name = lens.map_opt(olivier(), |name| name.to_uppercase());
        assert_eq!(mums_name, Some("ANNE".to_string()));
    }

    #[test]
    fn collections() {
        let lens = PersonParents.then(Every);
        let mut parents = lens.fold(olivier());
        assert_eq!(parents.next().unwrap().name, "Anne");
        assert_eq!(parents.next().unwrap().name, "Thierry");
        assert_eq!(parents.next(), None);
    }

    #[test]
    fn long_collections() {
        let lens = PersonParents.then(Every).then(PersonName);
        let mut parents = lens.fold(olivier());
        assert_eq!(parents.next().unwrap(), "Anne");
        assert_eq!(parents.next().unwrap(), "Thierry");
        assert_eq!(parents.next(), None);

        let lens = PersonParents
            .then(Filtered(|person: &Person| person.age < 56))
            .then(PersonName);
        let mut parents = lens.fold(olivier());
        assert_eq!(parents.next().unwrap(), "Anne");
        assert_eq!(parents.next(), None);

        let lens = PersonParents
            .then(Filtered(|person: &Person| person.age > 55))
            .then(PersonName);
        let mut parents = lens.fold(olivier());
        assert_eq!(parents.next().unwrap(), "Thierry");
        assert_eq!(parents.next(), None);

        let lens = PersonParents
            .then(Filtered(|person: &Person| person.age > 55))
            .then(PersonName);
        let new_olivier = lens.set(olivier(), |_t| "Mark".to_string());

        assert_eq!(new_olivier.parents[0].name, "Anne");
        assert_eq!(new_olivier.parents[1].name, "Mark");
    }
}
