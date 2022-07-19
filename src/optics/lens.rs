// mod tuple;

use super::{AffineFold, AffineTraversal, Fold, Getter, Setter, Traversal};

pub struct AsLens;
pub struct Lens<G, AT> {
    getter: G,
    aff_traversal: AT,
}

impl<G, AT> Lens<G, AT> {
    pub fn create<GA, ATA, S, Out, F, In>(getter: G, aff_traversal: AT) -> Self
    where
        G: Getter<GA, S>,
        AT: AffineTraversal<ATA, S, Out, F> + Fold<ATA, S, T = In>,
        F: FnMut(In) -> Out,
    {
        Self {
            getter,
            aff_traversal,
        }
    }
}

impl<A, S, G, AT> Getter<(AsLens, A), S> for Lens<G, AT>
where
    G: Getter<A, S>,
{
    type T = G::T;

    fn view(&self, source: S) -> <Self as Getter<(AsLens, A), S>>::T {
        self.getter.view(source)
    }
}

impl<A, S, G, AT, Out, F> AffineTraversal<(AsLens, A), S, Out, F> for Lens<G, AT>
where
    AT: AffineTraversal<A, S, Out, F>,
    F: FnMut(<AT as Fold<A, S>>::T) -> Out,
{
    fn map_opt(&self, source: S, f: F) -> Option<Out> {
        self.aff_traversal.map_opt(source, f)
    }
}

impl<A, S, G, AT, Out, F> Traversal<(AsLens, A), S, Out, F> for Lens<G, AT>
where
    AT: Traversal<A, S, Out, F>,
    F: FnMut(<AT as Fold<A, S>>::T) -> Out,
{
    type TraversalIter = AT::TraversalIter;

    fn map(&self, source: S, f: F) -> Self::TraversalIter {
        self.aff_traversal.map(source, f)
    }
}

impl<A, S, G, AT> AffineFold<(AsLens, A), S> for Lens<G, AT>
where
    AT: AffineFold<A, S>,
{
    fn preview(&self, source: S) -> Option<Self::T> {
        self.aff_traversal.preview(source)
    }
}

impl<A, S, G, AT> Setter<(AsLens, A), S> for Lens<G, AT>
where
    AT: Setter<A, S>,
{
    type In = AT::In;

    fn set<F>(&self, source: S, f: F) -> S
    where
        F: FnOnce(&mut Self::In),
    {
        self.aff_traversal.set(source, f)
    }
}

impl<A, S, G, AT> Fold<(AsLens, A), S> for Lens<G, AT>
where
    AT: Fold<A, S>,
{
    type T = AT::T;

    type FoldIter = AT::FoldIter;

    fn fold(&self, source: S) -> Self::FoldIter {
        self.aff_traversal.fold(source)
    }
}
#[cfg(test)]
mod tests {
    //     use crate::{
    //         data::{Person, Test},
    //         optics::Then,
    //     };

    use super::*;
    use crate::data::{Person, Test};

    //     fn is_lens<'a, L: LensLike<'a, S, G, M>, S, G, M>(_l: L) {}

    #[test]
    fn lens() {
        let mut test = Test("Foo".into());

        // let lens = Lens::create(Test::own_, Test::mut_);

        // assert_eq!(lens.view(test), "Foo");
        // lens.set(test, |f| *f = "Bar".into());

        let mut olivier = Person {
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
        };

        // let lens = Lens::create(Person::name, Person::name_mut);
        // let test = lens.set(olivier, |f| f);
        // assert_eq!(lens.view(test), "Bar");
        // assert_eq!(lens.preview(&test).unwrap(), "Bar");

        // is_lens(lens);
    }

    //     #[test]
    //     fn complex() {
    //         let mut olivier = Person {
    //             age: 24,
    //             name: "Olivier".into(),
    //             parents: vec![
    //                 Person {
    //                     age: 55,
    //                     name: "Anne".to_string(),
    //                     parents: vec![],
    //                 },
    //                 Person {
    //                     age: 56,
    //                     name: "Thierry".to_string(),
    //                     parents: vec![],
    //                 },
    //             ],
    //         };

    //         let name_lens = (Person::name, Person::name_mut, Person::name_opt);
    //         let mother_lens = (Person::mother, Person::mother_mut, Person::mother_opt);

    //         let mothers_name = mother_lens.then(name_lens);

    //         assert_eq!(mothers_name.view(&olivier), "Anne");
    //         mothers_name.set(&mut olivier, |name| *name = "Jocelyn".into());
    //         assert_eq!(mothers_name.view(&olivier), "Jocelyn");

    //         assert_eq!(mothers_name.preview(&olivier).unwrap(), "Jocelyn");

    //         let adam = Person {
    //             age: 1,
    //             name: "Adam".into(),
    //             parents: vec![],
    //         };

    //         assert_eq!(mothers_name.preview(&adam), None);
    //         is_lens(mothers_name);
    //     }

    //     #[test]
    //     fn manual() {
    //         let mut olivier = Person {
    //             age: 24,
    //             name: "Olivier".into(),
    //             parents: vec![
    //                 Person {
    //                     age: 55,
    //                     name: "Anne".to_string(),
    //                     parents: vec![],
    //                 },
    //                 Person {
    //                     age: 56,
    //                     name: "Thierry".to_string(),
    //                     parents: vec![],
    //                 },
    //             ],
    //         };

    //         let name_lens = PersonName;
    //         let mother_lens = (Person::mother, Person::mother_mut, Person::mother_opt);

    //         let mothers_name = mother_lens.then(name_lens);

    //         assert_eq!(mothers_name.view(&olivier), "Anne");
    //         mothers_name.set(&mut olivier, |name| *name = "Jocelyn".into());
    //         assert_eq!(mothers_name.view(&olivier), "Jocelyn");

    //         assert_eq!(mothers_name.preview(&olivier).unwrap(), "Jocelyn");

    //         let mothers_name = PersonMother.then(PersonName);

    //         assert_eq!(mothers_name.view(&olivier), "Jocelyn");
    //         mothers_name.set(&mut olivier, |name| *name = "Anne".into());
    //         assert_eq!(mothers_name.view(&olivier), "Anne");

    //         assert_eq!(mothers_name.preview(&olivier).unwrap(), "Anne");
    //         is_lens(mothers_name);
    //     }

    //     struct PersonName;
    //     impl<'a> GetLike<'a, Person, IsLens> for PersonName {
    //         type T = String;

    //         fn view(&self, source: &'a Person) -> &'a Self::T {
    //             &source.name
    //         }
    //     }
    //     impl<'a> SetLike<'a, Person, IsLens> for PersonName {
    //         type T = String;

    //         fn set<F>(&self, source: &'a mut Person, f: F)
    //         where
    //             F: FnOnce(&'a mut Self::T),
    //         {
    //             f(&mut source.name)
    //         }
    //     }
    //     // impl<'a> AffineFoldLike<'a, Person, IsLens> for PersonName {
    //     //     type T = String;

    //     //     fn preview(&self, source: &'a Person) -> Option<&'a Self::T> {
    //     //         Some(&source.name)
    //     //     }
    //     // }

    //     struct PersonMother;
    //     impl<'a> GetLike<'a, Person, IsLens> for PersonMother {
    //         type T = Person;

    //         fn view(&self, source: &'a Person) -> &'a Self::T {
    //             &source.parents[0]
    //         }
    //     }
    //     impl<'a> SetLike<'a, Person, IsLens> for PersonMother {
    //         type T = Person;

    //         fn set<F>(&self, source: &'a mut Person, f: F)
    //         where
    //             F: FnOnce(&'a mut Self::T),
    //         {
    //             f(&mut source.parents[0])
    //         }
    //     }
    //     // impl<'a> AffineFoldLike<'a, Person, IsLens> for PersonMother {
    //     //     type T = Person;

    //     //     fn preview(&self, source: &'a Person) -> Option<&'a Self::T> {
    //     //         source.parents.get(0)
    //     //     }
    //     // }
}
