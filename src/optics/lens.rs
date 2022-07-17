use std::marker::PhantomData;

use super::{GetLike, SetLike, TraversalLike};

mod ext;
mod lens_and_lens;
mod tuple;

pub use ext::LensExt;
pub use lens_and_lens::LensAndLens;

pub trait LensLike<'a, S, GM, SM, TM>:
    GetLike<'a, S, GM> + SetLike<'a, S, SM> + TraversalLike<'a, S, TM>
{
    #[cfg(test)]
    fn is_lens(&self) -> bool {
        true
    }
}

impl<'a, Src, GM1, SM1, TM1, L1> LensLike<'a, Src, GM1, SM1, TM1> for L1
where
    L1: GetLike<'a, Src, GM1> + SetLike<'a, Src, SM1> + TraversalLike<'a, Src, TM1>,
    Src: 'a,
{
}

pub struct IsLens;

pub struct Lens<L>(L);

impl<'a, Src, M1, L1> GetLike<'a, Src, (IsLens, M1)> for Lens<L1>
where
    L1: GetLike<'a, Src, M1>,
    Src: 'a,
{
    type T = L1::T;

    fn view(&self, source: &'a Src) -> &'a Self::T {
        self.0.view(source)
    }
}

impl<'a, Src, M1, L1> SetLike<'a, Src, (IsLens, M1)> for Lens<L1>
where
    L1: SetLike<'a, Src, M1>,
    Src: 'a,
{
    type T = L1::T;

    fn set<F>(&self, source: &'a mut Src, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        self.0.set(source, f)
    }
}

impl<'a, Src, M1, L1> TraversalLike<'a, Src, (IsLens, M1)> for Lens<L1>
where
    L1: TraversalLike<'a, Src, M1>,
    Src: 'a,
{
    type T = L1::T;

    fn preview(&self, source: &'a Src) -> Option<&'a Self::T> {
        self.0.preview(source)
    }
}

impl<'a, L1, L2> std::ops::Rem<Lens<L2>> for Lens<L1> {
    type Output = Lens<LensAndLens<L1, L2>>;

    fn rem(self, rhs: Lens<L2>) -> Self::Output {
        Lens(LensAndLens(self.0, rhs.0))
    }
}

impl<'a, L, S, GM> std::ops::BitXor<&'a S> for Lens<L>
where
    L: GetLike<'a, S, GM>,
{
    type Output = &'a L::T;

    fn bitxor(self, rhs: &'a S) -> Self::Output {
        self.0.view(rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Person, Test};

    use super::*;

    #[test]
    fn lens() {
        let mut test = Test("Foo".into());

        let lens = (Test::ref_, Test::mut_, Test::opt_);

        assert_eq!(lens.view(&test), "Foo");
        lens.set(&mut test, |f| *f = "Bar".into());
        assert_eq!(lens.view(&test), "Bar");
        assert_eq!(lens.preview(&test).unwrap(), "Bar");

        assert!(lens.is_lens());
    }

    #[test]
    fn complex_op() {
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

        let name_lens = (Person::name, Person::name_mut, Person::name_opt).into_lens();
        let mother_lens = (Person::mother, Person::mother_mut, Person::mother_opt).into_lens();

        let mothers_name = mother_lens % name_lens;

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
        assert!(mothers_name.is_lens());
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
        assert!(mothers_name.is_lens());
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
        assert!(mothers_name.is_lens());
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
