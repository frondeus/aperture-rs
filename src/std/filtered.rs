use crate::prelude::*;

#[derive(Clone)]
pub struct Filtered<Filter>(pub Filter);

impl<S, Filter> Traversal<S> for Filtered<Filter>
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
impl<S, Filter, T> TraversalMut<S> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a T) -> bool + Clone,
    S: IntoIterator<Item = T> + FromIterator<T>,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut T>,
{
    fn impl_set_mut<F>(&self, source: &mut S, mut f: F)
    where
        F: Clone + FnMut(&mut <Self::D as Iterator>::Item),
    {
        source.into_iter().for_each(|o| {
            if (self.0.clone())(o) {
                f(o);
            }
        });
    }
}
impl<S, Filter, T> TraversalRef<S> for Filtered<Filter>
where
    Filter: for<'a> FnMut(&'a T) -> bool + Clone,
    S: IntoIterator<Item = T> + FromIterator<T>,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut T>,
    for<'a> &'a S: IntoIterator<Item = &'a T>,
    for<'a> T: 'a,
    for<'a> S: 'a,
{
    type DRef<'a> = FilterRef<'a, T, Filter, <&'a S as IntoIterator>::IntoIter>;

    fn impl_fold_ref<'a>(&self, source: &'a S) -> Self::DRef<'a> {
        FilterRef {
            filter: self.0.clone(),
            iter: source.into_iter(),
        }
    }
}

pub struct FilterRef<'a, T, Filter, Iter>
where
    Iter: Iterator<Item = &'a T>,
    Filter: for<'b> FnMut(&'b T) -> bool + Clone,
    T: 'a,
{
    filter: Filter,
    iter: Iter,
}

impl<'a, T, Filter, Iter> Iterator for FilterRef<'a, T, Filter, Iter>
where
    Iter: Iterator<Item = &'a T>,
    Filter: for<'b> FnMut(&'b T) -> bool + Clone,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let t = self.iter.next()?;
            if !(self.filter)(t) {
                continue;
            } else {
                return Some(t);
            }
        }
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
    fn as_fold_ref() {
        let test: Vec<u32> = vec![1, 2, 3];

        let mut iter = Filtered(|x: &u32| x % 2 == 0).fold_ref(&test);
        assert_eq!(iter.next().unwrap(), &2);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_setter() {
        let test: Vec<u32> = vec![1, 2, 3];

        let new = Filtered(|x: &u32| x % 2 == 0).set(test, |x| x + 1);
        assert_eq!(new, vec![1, 3, 3]);
    }
}
