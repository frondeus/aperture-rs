pub trait FoldLike<'a, S, Marker>
where
    S: ?Sized,
{
    type T: 'a;
    type Iter: Iterator<Item = &'a Self::T>;

    fn fold(&self, source: &'a S) -> Self::Iter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        let mut iter = <[String]>::iter.fold(&test);
        assert_eq!(iter.next().unwrap(), "Foo");
    }
}
