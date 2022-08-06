use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsTraversal;
pub trait Traversal<As, S> // where
{
    type D: Iterator; // = S::IntoIter;
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<Self::D, F>
    where
        F: FnMut(<Self::D as Iterator>::Item) -> T,
    {
        self.impl_fold(source).map(f)
    }

    #[doc(hidden)]
    fn impl_fold(&self, source: S) -> Self::D;

    #[doc(hidden)]
    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(<Self::D as Iterator>::Item) -> <Self::D as Iterator>::Item;
}

// #[cfg(feature = "gat")]
pub trait TraversalRef<As, S>: TraversalMut<As, S> {
    type Item<'a>: 'a
    where
        S: 'a;

    type DRef<'a>: Iterator<Item = &'a Self::Item<'a>>
    where
        S: 'a;

    fn impl_fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a>;

    fn traverse_ref<'a, F, T>(&self, source: &'a S, f: F) -> std::iter::Map<Self::DRef<'a>, F>
    where
        F: FnMut(&'a Self::Item<'a>) -> T,
    {
        self.impl_fold_ref(source).map(f)
    }
}

pub trait TraversalMut<As, S>: Traversal<As, S> {
    #[doc(hidden)]
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item);
}

impl<S, X> Optics<AsTraversal, S> for X where X: Traversal<AsTraversal, S> {}

impl<X, S> Fold<AsTraversal, S> for X
where
    X: Traversal<AsTraversal, S>,
    S: IntoIterator + FromIterator<S::Item>,
{
    type D = X::D;

    fn fold(&self, source: S) -> Self::D {
        Traversal::impl_fold(self, source)
    }
}

impl<X, S> Setter<AsTraversal, S> for X
where
    X: Traversal<AsTraversal, S>,
{
    type O = <X::D as Iterator>::Item;
    fn set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(Self::O) -> Self::O + Clone,
    {
        self.impl_set(source, f)
    }
}

impl<X, S> SetterMut<AsTraversal, S> for X
where
    X: TraversalMut<AsTraversal, S>,
{
    fn set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut Self::O) + Clone,
    {
        self.impl_set_mut(source, f);
    }
}

// #[cfg(feature = "gat")]
impl<X, S> FoldRef<AsTraversal, S> for X
where
    X: TraversalRef<AsTraversal, S>,
    S: IntoIterator + FromIterator<S::Item>,
{
    type Item<'a> = X::Item<'a> where S: 'a;

    type DRef<'a> = X::DRef<'a> where S: 'a;

    fn fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        self.impl_fold_ref(source)
    }
}

macro_rules! impl_and {
 ($as: ident, $(($l:ident, $r:ident),)*) => { impl_and!(@ ($as, $as), $(($l, $r), ($r, $l)),*); };
 (@ $(($l:ident, $r:ident)),*) => {$(
impl<L1, L2, S> Traversal<AsTraversal, S>
    for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
where
    L1: Traversal<$l, S>,
    L2: Clone + Traversal<$r, <L1::D as Iterator>::Item>,
{
    type D = nested::NestedTraversal<$r, L1::D, L2>;

    fn impl_fold(&self, source: S) -> Self::D {
        nested::NestedTraversal::new(self.0.impl_fold(source), self.1.clone())
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: Clone + FnMut(<Self::D as Iterator>::Item) -> <Self::D as Iterator>::Item,
    {
        self.0.impl_set(source, |x| self.1.impl_set(x, f.clone()))
    }
}
impl<L1, L2, S> TraversalMut<AsTraversal, S>
    for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
where
    L1: TraversalMut<$l, S>,
    L2: Clone + TraversalMut<$r, <L1::D as Iterator>::Item>,
{
    fn impl_set_mut<F>(&self, source: &mut S, f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item),
    {
        self.0.impl_set_mut(source, |x| self.1.impl_set_mut(x, f.clone()))
    }
}
impl<L1, L2, S> TraversalRef<AsTraversal, S>
    for And<L1, L2, ($l, $r), (S, <L1::D as Iterator>::Item)>
where
    L1: TraversalMut<$l, S>,
    L2: Clone + TraversalMut<$r, <L1::D as Iterator>::Item>,

    L1: TraversalRef<$l, S>,
    for<'a> L2: Clone + TraversalRef<$r, L1::Item<'a>>,
    for<'a> S: 'a,
{
    type Item<'a> = <L2 as TraversalRef<$r, L1::Item<'a>>>::Item<'a>;

    type DRef<'a> = nested::NestedTraversalRef<
        'a,
        $l,
        $r,
        L1,
        L2,
        S
    >;

    fn impl_fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        nested::NestedTraversalRef::new(self.0.impl_fold_ref(source), self.1.clone())
    }
}
 )*};
}

impl_and!(
    AsTraversal,
    (AsTraversal, AsAffineTraversal),
    (AsTraversal, AsLens),
    (AsTraversal, AsPrism),
    // (AsTraversal, AsIso),
);

mod nested;
pub use nested::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::Person,
        prelude::{
            every::Every,
            filtered::Filtered,
            person_at::PersonMotherAT,
            person_folds::PersonParentsFold,
            person_setters::PersonNameSetter,
            Then,
        },
    };

    #[test]
    fn traversal_and_traversal() {
        let lens = Every.then(Filtered(|x: &i32| *x % 2 == 0));
        let src = vec![vec![1, 2, 3]];
        let res = lens.traverse(src, |x| x).collect::<Vec<_>>();
        assert_eq!(res, vec![2]);

        let lens = Every.then(Filtered(|x: &i32| *x % 2 == 0));
        let src = vec![vec![1, 2, 3]];
        let res = lens.set(src, |x| x + 8);
        assert_eq!(res, vec![vec![1, 10, 3]]);
    }
    #[test]
    fn traversal_and_traversal_mut() {
        let lens = Every.then(Filtered(|x: &i32| *x % 2 == 0));
        let mut src = vec![vec![1, 2, 3]];
        lens.set_mut(&mut src, |x| *x = *x + 8);
        assert_eq!(src, vec![vec![1, 10, 3]]);
    }
    #[test]
    fn traversal_and_traversal_ref() {
        let lens = Every.then(Filtered(|x: &i32| *x % 2 == 0));
        let src = vec![vec![1, 2, 3]];
        let res: Vec<_> = lens.traverse_ref(&src, |x| *x + 8).collect();
        assert_eq!(res, vec![10]);
    }

    #[test]
    fn and_is_valid_traversal() {
        let lens = Every.then(Every).then(Filtered(|x: &i32| *x % 2 == 0));
        let src = vec![vec![vec![1, 2, 3]]];
        let mut res = lens.traverse(src, |x: i32| x);
        assert_eq!(res.next(), Some(2))
    }

    #[test]
    fn and_is_valid_traversal_ref() {
        let lens = Every.then(Every).then(Filtered(|x: &i32| *x % 2 == 0));
        let src = vec![vec![vec![1, 2, 3]]];
        let mut res = lens.traverse_ref(&src, |x: &i32| x);
        assert_eq!(res.next(), Some(&2))
    }

    #[test]
    fn traversal_and_setter() {
        let lens = Every.then(PersonNameSetter);

        let src = vec![Person::olivier(), Person::wojtek()];

        let res: Vec<Person> = lens.set(src, |name| name.to_uppercase());
        let res = res.into_iter().map(|p| p.name).collect::<Vec<_>>();
        assert_eq!(res, vec!["OLIVIER", "WOJTEK"]);
    }

    #[test]
    fn traversal_and_fold() {
        let lens = Every.then(PersonParentsFold);

        let src = vec![Person::olivier(), Person::wojtek()];

        let res = lens.fold(src);
        let res = res.map(|p| p.name).collect::<Vec<_>>();
        assert_eq!(res, vec!["Anne", "Thierry", "Miroslawa", "Zenon"]);
    }

    #[test]
    fn traversal_and_at() {
        let lens = Every.then(PersonMotherAT);

        let src = vec![Person::olivier(), Person::wojtek()];

        let res: Vec<String> = Traversal::traverse(&lens, src, |x| x.name).collect();
        assert_eq!(res, vec!["Anne", "Miroslawa"]);

        let lens = Every.then(PersonMotherAT);

        let src = vec![Person::olivier(), Person::wojtek()];

        let res = Setter::set(&lens, src, |mut x| {
            x.name = x.name.to_uppercase();
            x
        });
        assert_eq!(res[0].parents[0].name, "ANNE");
        assert_eq!(res[0].parents[1].name, "Thierry");
        assert_eq!(res[1].parents[0].name, "MIROSLAWA");
        assert_eq!(res[1].parents[1].name, "Zenon");
    }
}
