use crate::method::Method;

pub struct AsFoldMethod;
pub trait Fold<As, S>
where
    S: ?Sized,
{
    type T;
    type FoldIter: Iterator<Item = Self::T>;

    fn fold(&self, source: S) -> Self::FoldIter;
}
// Since fold is basically traversal with identity function it is automatically implemented
// impl<T, M, I, S, Any> Fold<Any, S> for M
// where
//     M: Method<S, (), Output = I>,
//     I: Iterator<Item = T>,
// {
//     type T = T;

//     type FoldIter = I;

//     fn fold(&self, source: S) -> Self::FoldIter {
//         self.mcall(source, ())
//     }
// }

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

        // assert_fold::<AsFold, _, _, _>(Vec::<String>::into_iter);

        let mut iter = IntoIterator::into_iter.fold(test);
        assert!(iter.next().unwrap() == "Foo".to_string());
    }
}
