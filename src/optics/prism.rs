use super::{AffineFold, ReviewLike, SetLike};

pub trait PrismLike<'a, S, TM>: ReviewLike<'a, S> + AffineFold<'a, S, TM> {}

impl<'a, S, TM, P> PrismLike<'a, S, TM> for P where P: ReviewLike<'a, S> + AffineFold<'a, S, TM> {}

pub struct IsPrism;

pub struct At<T>(pub T);

impl<'a, S> ReviewLike<'a, Vec<S>> for At<usize>
where
    S: 'a,
{
    type T = S;

    fn review(&self, mut source: Vec<S>) -> Self::T {
        source.swap_remove(self.0)
    }
}

impl<'a, S> AffineFold<'a, Vec<S>, IsPrism> for At<usize>
where
    S: 'a,
{
    type T = S;

    fn preview(&self, source: &'a Vec<S>) -> Option<&'a Self::T> {
        source.get(self.0)
    }
}

impl<'a, S, SM> SetLike<'a, Vec<S>, SM> for At<usize>
where
    S: 'a,
{
    type T = S;

    fn set<F>(&self, source: &'a mut Vec<S>, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        if let Some(t) = source.get_mut(self.0) {
            f(t)
        }
    }
}

pub trait PrismVecExt<T> {
    fn at(index: usize) -> At<usize>;
}

impl<T> PrismVecExt<T> for Vec<T> {
    fn at(index: usize) -> At<usize> {
        At(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::Person,
        optics::{LensLike, Then},
    };

    use super::*;

    fn is_lens<'a, L: LensLike<'a, S, G, M, T>, S, G, M, T>(_l: L) {}
    fn is_person_prism<'a, P>(_p: P)
    where
        P: PrismLike<'a, Vec<Person>, IsPrism>,
    {
    }

    #[test]
    fn at() {
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

        let parents_lens = (Person::parents, Person::parents_mut, Person::parents_opt);
        let name_lens = (Person::name, Person::name_mut, Person::name_opt);

        is_lens(parents_lens);
        is_person_prism(At(0));
        let res = parents_lens.then(At(0));
        // let mothers_name = parents_lens.then(Vec::<Person>::at(0)).then(name_lens);

        // assert_eq!(mothers_name.preview(&olivier).unwrap(), "Anne");
    }
}
