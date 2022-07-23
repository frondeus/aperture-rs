use crate::prelude::And;

pub struct AsSetter;
pub trait Setter<As, S> {
    type O;
    type D;
    type T;
    fn set<F>(&self, source: S, f: F) -> Self::D
    where
        F: FnMut(Self::O) -> Self::T + Clone;
}

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

// impl<S, M, T> Setter<AsSetter, S> for M
// where
//     M: for<'a> crate::method::Method<&'a mut S, (), Output = &'a mut T>,
// {
//     type O = T;

//     type D = S;

//     type T = T;

//     fn set<F>(&self, mut source: S, f: F) -> Self::D
//     where
//         F: FnMut(Self::O) -> Self::T + Clone,
//     {
//         let mutable = self.mcall(&mut source, ());
//         take_mut::take(mutable, f);
//         source
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            lenses::{PersonMother, PersonName},
            Person,
        },
        prelude::*,
    };

    // #[test]
    // fn method() {
    //     let lens = PersonMother.then(Person::name_mut);
    //     let new_olivier = lens.set(Person::olivier(), |name| name.to_uppercase());

    //     assert_eq!(new_olivier.parents[0].name, "ANNE");
    // }

    #[test]
    fn combinator() {
        let lens: And<PersonMother, PersonName> = PersonMother.then(PersonName);
        let new_olivier = lens.set(Person::olivier(), |name| name.to_uppercase());
        assert_eq!(new_olivier.parents[0].name, "ANNE");
    }
}
