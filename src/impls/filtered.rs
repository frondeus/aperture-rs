use crate::prelude::*;

#[derive(Clone)]
pub struct Filtered<Filter>(pub Filter);

impl<S, Filter> Traversal<AsTraversal, S> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a S::Item) -> bool + Clone,
    S: IntoIterator + FromIterator<S::Item>,
{
    type D = std::iter::Filter<S::IntoIter, Filter>;
    fn impl_fold(&self, source: S) -> Self::D {
        source.into_iter().filter(self.0.clone())
    }

    fn impl_set<F>(&self, source: S, mut f: F) -> S
    where
        F: FnMut(<Self::D as Iterator>::Item) -> <Self::D as Iterator>::Item,
    {
        source
            .into_iter()
            .map(|o| match (self.0.clone())(&o) {
                true => f(o),
                false => o,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_traversal() {
        let test: Vec<u32> = vec![1, 2, 3];

        let mut iter = Filtered(|x: &u32| x % 2 == 0).traverse(test, |x| x + 1);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_fold() {
        let test: Vec<u32> = vec![1, 2, 3];

        let mut iter = Filtered(|x: &u32| x % 2 == 0).fold(test);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_setter() {
        let test: Vec<u32> = vec![1, 2, 3];

        let new = Filtered(|x: &u32| x % 2 == 0).set(test, |x| x + 1);
        assert_eq!(new, vec![1, 3, 3]);
    }
}
