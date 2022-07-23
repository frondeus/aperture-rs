// mod tuple;

use super::{AffineFold, AffineTraversal, Fold, Getter, Setter, Traversal};

pub struct AsLens;
pub trait Lens<As, S>
where
    Self: Getter<As, S> + AffineTraversal<As, S>,
    Self::D: Iterator,
{
}

impl<As, L, S> Lens<As, S> for L
where
    L: Getter<As, S> + AffineTraversal<As, S>,
    L::D: Iterator,
{
}

#[cfg(test)]
mod tests {
    //     use crate::{
    //         data::{Person, Test},
    //         optics::Then,
    //     };

    use super::*;
    use crate::data::{
        lenses::{PersonMother, PersonParents},
        Person,
        Test,
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
        let mom = PersonMother.view(olivier());
        assert_eq!(&mom.name, "Anne");

        let parents = PersonParents.view(olivier());
        assert_eq!(&parents[1].name, "Thierry");
    }

    #[test]
    fn as_setter() {
        let new_olivier = PersonMother.set(olivier(), |mut mom| {
            mom.name = "Jocelyn".into();
            mom
        });
        let mom = PersonMother.view(new_olivier);
        assert_eq!(&mom.name, "Jocelyn");

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
