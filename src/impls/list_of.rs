use crate::prelude::*;

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
}
