use crate::prelude::*;

#[derive(Clone)]
pub struct ListOf;

impl<S> Fold<AsFold, S> for ListOf
where
    S: IntoIterator,
{
    type D = S::IntoIter;

    fn fold(&self, source: S) -> Self::D {
        source.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        let mut iter = ListOf.fold(test);
        assert_eq!(iter.next().unwrap(), "Foo");
    }

    #[test]
    fn fold_ref() {
        let test: Vec<String> = vec!["Foo".into()];
        let mut iter = ListOf.fold(&test);
        let item = iter.next().unwrap();
        assert_eq!(item, "Foo");
    }

    #[test]
    fn fold_mut() {
        let mut test: Vec<String> = vec!["Foo".into()];
        let mut iter = ListOf.fold(&mut test);
        let item = iter.next().unwrap();
        assert_eq!(item, "Foo");
    }
}
