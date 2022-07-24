// mod tuple;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct AsLens;
pub trait Lens<As, S>
where
    Self: Getter<As, S> + AffineTraversal<As, S>,
    <Self as Fold<As, S>>::D: Iterator,
{
}

impl<As, L, S> Lens<As, S> for L
where
    L: Getter<As, S> + AffineTraversal<As, S>,
    <L as Fold<As, S>>::D: Iterator,
{
}

pub trait DerivedLens<S> {
    type View;
    fn derived_view(&self, source: S) -> Self::View;
    fn derived_set<F: FnMut(Self::View) -> Self::View>(&self, source: S, f: F) -> S;
}

impl<L, S> Optics<AsLens, S> for L where L: DerivedLens<S> {}
impl<L, S> Getter<AsLens, S> for L
where
    L: DerivedLens<S>,
{
    type T = L::View;

    fn view(&self, source: S) -> <Self as Getter<AsLens, S>>::T {
        DerivedLens::derived_view(self, source)
    }
}
impl<L, S> AffineFold<AsLens, S> for L
where
    L: DerivedLens<S>,
{
    type T = L::View;

    fn preview(&self, source: S) -> Option<<Self as AffineFold<AsLens, S>>::T> {
        Some(DerivedLens::derived_view(self, source))
    }
}
impl<L, S> Fold<AsLens, S> for L
where
    L: DerivedLens<S>,
{
    type D = std::option::IntoIter<L::View>;

    fn fold(&self, source: S) -> Self::D {
        AffineFold::preview(self, source).into_iter()
    }
}
impl<L, S> Setter<AsLens, S> for L
where
    L: DerivedLens<S>,
{
    type O = L::View;

    type D = S;

    type T = L::View;

    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone,
    {
        DerivedLens::derived_set(self, source, f)
    }
}
impl<L, S> Traversal<AsLens, S> for L
where
    L: DerivedLens<S>,
{
    fn traverse<F, T>(&self, source: S, f: F) -> std::iter::Map<<Self as Fold<AsLens, S>>::D, F>
    where
        F: FnMut(<<Self as Fold<AsLens, S>>::D as Iterator>::Item) -> T,
    {
        self.fold(source).map(f)
    }
}
impl<L, S> AffineTraversal<AsLens, S> for L
where
    L: DerivedLens<S>,
{
    fn map_opt<T, F>(&self, source: S, f: F) -> Option<T>
    where
        F: FnOnce(<<Self as Fold<AsLens, S>>::D as Iterator>::Item) -> T,
    {
        self.preview(source).map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        lenses::{PersonMother, PersonMother2, PersonParents},
        Person,
    };

    #[test]
    fn as_view() {
        let mom = PersonMother.view(Person::olivier());
        assert_eq!(&mom.name, "Anne");

        let parents = PersonParents.view(Person::olivier());
        assert_eq!(&parents[1].name, "Thierry");

        // let lens = PersonMother2::default();
        // let lens = PersonParents.then(first::First); //.then(unwrap::Unwrap);
        //                                              // dbg!(&lens);
        //                                              // todo!();
        // let mom = lens.preview(Person::olivier());

        let every_parent = PersonParents.then(every::Every);

        let parents = every_parent.fold(Person::olivier()).collect::<Vec<_>>();
        dbg!(&parents);

        let first_parent_opt = PersonParents.then(first::First); // fold, aff trav, aff fold, setter, traversal
        let first_parent = first_parent_opt.then(unwrap::Unwrap);
        // first_parent.
        // first_parent.preview(Person::olivier());
        todo!()

        // assert_eq!(&mom.name, "Anne");
    }

    #[test]
    fn as_setter() {
        let new_olivier = PersonMother.derived_set(Person::olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        let mom = PersonMother.view(new_olivier);
        assert_eq!(&mom.name, "Jocelyn");
    }
}
