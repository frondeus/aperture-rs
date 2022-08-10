pub struct PersonMother;
impl Lens<AsLens, Person> for PersonMother {
    type View = Person;

    fn impl_view(&self, source: Person) -> Self::View {
        // Actually Person Mother should be a telescope
        Lens::impl_preview(self, source).unwrap()
    }

    fn impl_set<F: FnMut(Self::View) -> Self::View>(&self, mut source: Person, f: F) -> Person {
        let mut iter = source.parents.into_iter();
        let new_mom = iter.next().map(f);
        source.parents = new_mom.into_iter().chain(iter).collect();
        source
    }

    fn impl_preview(&self, source: Person) -> Option<Self::View> {
        source.parents.into_iter().next()
    }
}

impl LensMut<AsLens, Person> for PersonMother {
    fn impl_set_mut<F: Clone + FnMut(&mut Self::View)>(&self, source: &mut Person, f: F) {
        let mut iter = source.parents.iter_mut();
        iter.next().map(f);
    }
}
impl LensRef<AsLens, Person> for PersonMother {
    fn impl_view_ref<'a>(&self, source: &'a Person) -> &'a Self::View {
        LensRef::impl_preview_ref(self, source).unwrap()
    }

    fn impl_preview_ref<'a>(&self, source: &'a Person) -> Option<&'a Self::View> {
        source.parents.first()
    }
}

use crate::{
    data::{Person, *},
    prelude::*,
};
pub type PersonName = PersonNameLens;
pub type PersonParents = PersonParentsLens;

#[allow(non_upper_case_globals)]
impl Person {
    pub const mother: PersonMother = PersonMother;
}

pub trait PersonLensesExt<S>: Lens<AsLens, S> + Sized {
    fn then_mother(self) -> And<Self, PersonMother, (AsLens, AsLens), (S, Person)> {
        self.then(PersonMother)
    }
}
impl<L, S> PersonLensesExt<S> for L where L: Lens<AsLens, S, View = Person> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_mut() {
        let lens = PersonMother;
        let mut wojtek = Person::wojtek();
        lens.set_mut(&mut wojtek, |mom| {
            mom.name = mom.name.to_uppercase();
        });
        let mom = &wojtek.parents[0].name;
        assert_eq!(mom, "MIROSLAWA");

        Person::name.set_mut(&mut wojtek, |name| {
            *name = "Philip".into();
        });
        assert_eq!(wojtek.name, "Philip");
    }

    #[test]
    fn as_af_no_mom() {
        let lens = PersonMother;
        let mut wojtek = Person::wojtek();
        wojtek.parents.clear();

        assert_eq!(lens.preview_ref(&wojtek), None);
        assert_eq!(lens.preview(wojtek), None);
    }
}
