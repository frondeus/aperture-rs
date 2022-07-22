use crate::method::Method;

pub struct AsSetterMethod;
pub trait Setter<As, S, T> {
    type O;
    type D;
    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> T + Clone;
}

impl<S, D, M, T> Setter<AsSetterMethod, S, T> for M
where
    M: Method<S, (T,), Output = D>,
{
    type D = D;
    type O = ();

    fn set<F>(&self, source: S, mut f: F) -> Self::D
    where
        F: FnMut(Self::O) -> T + Clone,
    {
        let new = f(());
        self.mcall(source, (new,))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{Arg, Person, Test},
        lazy::LazyExt,
    };

    fn is_setter_a_method<M, In, S>(m: M)
    where
        M: Method<S, (In,), Output = S>,
    {
    }

    #[test]
    fn set() {
        let olivier = Person {
            age: 24,
            name: "Olivier".into(),
            parents: vec![],
        };

        is_setter_a_method(Person::set_name);
        let new = Person::set_name.set(olivier, |()| "new".to_string());
        assert_eq!(new.name, "new");
        assert_eq!(new.age, 24);

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
