use super::And;

pub struct AsSetterMethod;
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
// impl<S, D, M, T> Setter<AsSetterMethod, S> for M
// where
//     M: Method<S, (T,), Output = D>,
// {
//     type D = D;
//     type O = ();
//     type T = T;

//     fn set<F>(&self, source: S, mut f: F) -> Self::D
//     where
//         F: FnMut(Self::O) -> Self::T + Clone,
//     {
//         let new = f(());
//         self.mcall(source, (new,))
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
        optics::Then,
    };

    #[test]
    fn combinator() {
        let lens: And<PersonMother, PersonName> = PersonMother.then(PersonName);
        let new_olivier = lens.set(Person::olivier(), |name| name.to_uppercase());
        assert_eq!(new_olivier.mother().name, "ANNE");
    }
}
