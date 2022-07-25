use crate::prelude::*;

#[derive(Clone)]
pub struct Every;

impl<S> Traversal<AsTraversal, S> for Every
where
    S: IntoIterator + FromIterator<S::Item>,
{
    type D = S::IntoIter;
    fn impl_fold(&self, source: S) -> S::IntoIter {
        source.into_iter()
    }

    fn impl_set<F>(&self, source: S, f: F) -> S
    where
        F: FnMut(<Self::D as Iterator>::Item) -> <Self::D as Iterator>::Item,
    {
        self.traverse(source, f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_traversal() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Every.traverse(test, |x: String| x.to_uppercase());
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_fold() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let mut iter = Every.fold(test);
        assert_eq!(iter.next().unwrap(), "foo");
        assert_eq!(iter.next().unwrap(), "bar");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_setter() {
        let test: Vec<String> = vec!["foo".into(), "bar".into()];

        let new_test = Every.set(test, |t| t.to_uppercase());
        let mut iter = new_test.into_iter();
        assert_eq!(iter.next().unwrap(), "FOO");
        assert_eq!(iter.next().unwrap(), "BAR");
        assert_eq!(iter.next(), None);
    }
}
