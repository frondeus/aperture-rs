use super::{GetLike, SetLike, TraversalLike};

mod ext;
mod lens_and_lens;
mod lens_and_prism {}
mod traversal_and_lens {}
mod prism_and_review {}
mod tuple;

pub use ext::{IntoLens, Then};
pub use lens_and_lens::LensAndLens;

pub trait LensLike<'a, S, GM, SM, TM>:
    GetLike<'a, S, GM> + SetLike<'a, S, SM> + TraversalLike<'a, S, TM>
{
}

impl<'a, Src, GM1, SM1, TM1, L1> LensLike<'a, Src, GM1, SM1, TM1> for L1
where
    L1: GetLike<'a, Src, GM1> + SetLike<'a, Src, SM1> + TraversalLike<'a, Src, TM1>,
    Src: 'a,
{
}

pub struct IsLens;

#[cfg(test)]
mod tests {
    use crate::data::{Person, Test};

    use super::*;

    fn is_lens<'a, L: LensLike<'a, S, G, M, T>, S, G, M, T>(_l: L) {}

    #[test]
    fn lens() {
        let mut test = Test("Foo".into());

        let lens = (Test::ref_, Test::mut_, Test::opt_);

        assert_eq!(lens.view(&test), "Foo");
        lens.set(&mut test, |f| *f = "Bar".into());
        assert_eq!(lens.view(&test), "Bar");
        assert_eq!(lens.preview(&test).unwrap(), "Bar");

        is_lens(lens);
    }

    #[test]
    fn complex() {
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

        let name_lens = (Person::name, Person::name_mut, Person::name_opt);
        let mother_lens = (Person::mother, Person::mother_mut, Person::mother_opt);

        let mothers_name = mother_lens.then(name_lens);

        assert_eq!(mothers_name.view(&olivier), "Anne");
        mothers_name.set(&mut olivier, |name| *name = "Jocelyn".into());
        assert_eq!(mothers_name.view(&olivier), "Jocelyn");

        assert_eq!(mothers_name.preview(&olivier).unwrap(), "Jocelyn");

        let adam = Person {
            age: 1,
            name: "Adam".into(),
            parents: vec![],
        };

        assert_eq!(mothers_name.preview(&adam), None);
        is_lens(mothers_name);
    }

    #[test]
    fn manual() {
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

        let name_lens = PersonName;
        let mother_lens = (Person::mother, Person::mother_mut, Person::mother_opt);

        let mothers_name = mother_lens.then(name_lens);

        assert_eq!(mothers_name.view(&olivier), "Anne");
        mothers_name.set(&mut olivier, |name| *name = "Jocelyn".into());
        assert_eq!(mothers_name.view(&olivier), "Jocelyn");

        assert_eq!(mothers_name.preview(&olivier).unwrap(), "Jocelyn");

        let mothers_name = PersonMother.then(PersonName);

        assert_eq!(mothers_name.view(&olivier), "Jocelyn");
        mothers_name.set(&mut olivier, |name| *name = "Anne".into());
        assert_eq!(mothers_name.view(&olivier), "Anne");

        assert_eq!(mothers_name.preview(&olivier).unwrap(), "Anne");
        is_lens(mothers_name);
    }

    struct PersonName;
    impl<'a> GetLike<'a, Person, IsLens> for PersonName {
        type T = String;

        fn view(&self, source: &'a Person) -> &'a Self::T {
            &source.name
        }
    }
    impl<'a> SetLike<'a, Person, IsLens> for PersonName {
        type T = String;

        fn set<F>(&self, source: &'a mut Person, f: F)
        where
            F: FnOnce(&'a mut Self::T),
        {
            f(&mut source.name)
        }
    }
    impl<'a> TraversalLike<'a, Person, IsLens> for PersonName {
        type T = String;

        fn preview(&self, source: &'a Person) -> Option<&'a Self::T> {
            Some(&source.name)
        }
    }

    struct PersonMother;
    impl<'a> GetLike<'a, Person, IsLens> for PersonMother {
        type T = Person;

        fn view(&self, source: &'a Person) -> &'a Self::T {
            &source.parents[0]
        }
    }
    impl<'a> SetLike<'a, Person, IsLens> for PersonMother {
        type T = Person;

        fn set<F>(&self, source: &'a mut Person, f: F)
        where
            F: FnOnce(&'a mut Self::T),
        {
            f(&mut source.parents[0])
        }
    }
    impl<'a> TraversalLike<'a, Person, IsLens> for PersonMother {
        type T = Person;

        fn preview(&self, source: &'a Person) -> Option<&'a Self::T> {
            source.parents.get(0)
        }
    }
}
