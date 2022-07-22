use crate::method::Method;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        let mut iter = ListOf.fold(test);
        assert!(iter.next().unwrap() == "Foo".to_string());

        let test: Vec<u32> = vec![1, 2, 3];
        let folded = FoldOf(|x, y| x + y, || 0).fold(test);
        assert!(folded == 6);
        // assert_fold::<AsFold, _, _, _>(Vec::<String>::into_iter);

        // let iter: Vec<String> = Vec::<String>::into_iter.fold(test);
    }
}
