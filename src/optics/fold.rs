pub trait FoldLike<S, Marker>
where
    S: ?Sized,
{
    type T;
    type Iter: Iterator<Item = Self::T>;

    fn fold(&self, source: S) -> Self::Iter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        let mut iter = Vec::<String>::into_iter.fold(test);
        assert_eq!(iter.next().unwrap(), "Foo");
    }
}
