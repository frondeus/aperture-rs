use super::And;

pub struct AsFoldMethod;
pub trait Fold<As, S> {
    type D;

    fn fold(&self, source: S) -> Self::D;
}

pub mod nested;
// Since fold is basically traversal with identity function it is automatically implemented
// impl<T, M, I, S> Fold<AsFoldMethod, S> for M
// where
//     M: Method<S, (), Output = I>,
//     I: FromIterator<T> + IntoIterator<Item = T>,
// {
//     type T = T;
//     type Iter = I;

//     fn fold(&self, source: S) -> Self::Iter {
//         self.mcall(source, ()).collect()
//     }
// }

pub struct FoldOf<F, TF>(F, TF);
impl<S, F, T, TF> Fold<AsFoldMethod, S> for FoldOf<F, TF>
where
    S: IntoIterator<Item = T>,
    F: FnMut(T, T) -> T,
    TF: Fn() -> T,
    F: Copy,
{
    type D = T;

    fn fold(&self, source: S) -> Self::D {
        let t = (self.1)();
        source.into_iter().fold(t, self.0)
    }
}

pub struct ListOf;
impl<S> Fold<AsFoldMethod, S> for ListOf
where
    S: IntoIterator,
{
    type D = S::IntoIter;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter()
    }
}
// #[cfg(test)]
// pub fn assert_fold<As, Optic, S>(_o: Optic)
// where
//     Optic: Fold<As, S>,
// {
// }
impl<A1, A2, L1, L2, S> Fold<(A1, A2), S> for And<L1, L2>
where
    L1: Fold<A1, S>,
    L1::D: Iterator,
    L2: Fold<A2, <<L1 as Fold<A1, S>>::D as Iterator>::Item> + Clone,
    L2::D: Iterator,
{
    type D = nested::NestedFold<A2, L1::D, L2>;

    fn fold(&self, source: S) -> Self::D {
        nested::NestedFold::new(self.0.fold(source), self.1.clone())
    }
}

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
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        let mut iter = ListOf.fold(test);
        assert_eq!(iter.next().unwrap(), "Foo");

        let test: Vec<u32> = vec![1, 2, 3];
        let folded = FoldOf(|x, y| x + y, || 0).fold(test);
        assert!(folded == 6);
        // assert_fold::<AsFold, _, _, _>(Vec::<String>::into_iter);

        // let iter: Vec<String> = Vec::<String>::into_iter.fold(test);
    }

    #[test]
    fn combinator() {
        let lens = PersonMother.then(PersonName);
        let mut iter = Fold::fold(&lens, Person::olivier());
        let mums_name = iter.next();
        assert_eq!(mums_name, Some("Anne".to_string()));
    }
}
