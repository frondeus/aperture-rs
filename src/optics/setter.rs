use crate::method::Method;

pub struct AsSetterMethod;
pub trait Setter<As, S> {
    type In;
    fn set<F>(&self, source: S, f: F) -> S
    where
        F: FnOnce(&mut Self::In);
}

impl<S, M, T> Setter<AsSetterMethod, S> for M
where
    M: for<'a> Method<&'a mut S, (), Output = &'a mut T>,
{
    type In = T;

    fn set<F>(&self, mut source: S, f: F) -> S
    where
        F: FnOnce(&mut Self::In),
    {
        let mut _mut = self.mcall(&mut source, ());
        f(_mut);
        source
    }
}

// #[cfg(test)]
// pub fn assert_setter<Optic, S, As, M>(_o: Optic)
// where
//     Optic: Setter<As, S>,
// {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{Arg, Person, Test},
        lazy::LazyExt,
    };

    #[test]
    fn set() {
        let test = Test("Foo".into());

        let test = Test::mut_.set(test, |x| {
            *x = "Bar".into();
        });
        assert_eq!(test.0, "Bar");

        let olivier = Person {
            age: 24,
            name: "Olivier".into(),
            parents: vec![],
        };
        let olivier2 = Person::name_mut.set(olivier, |name| *name = "New Olivier".into());
        assert_eq!(olivier2.name, "New Olivier");
        // let test = Test::mut_arg
        //     .with_args((1,))
        //     .set(test, |x| *x = "Bar".into());
        // assert_eq!(test.0, "Bar");
        // let test = Test::mut_complex
        //     .lazy(|| (Arg,))
        //     .set(test, |x| *x = "Bar".into());
        // assert_eq!(test.0, "Bar");
    }
}
