pub struct AsFold;
pub trait Fold<As, S>
where
    S: ?Sized,
{
    type T;
    type Iter: Iterator<Item = Self::T>;

    fn fold(&self, source: S) -> Self::Iter;
}

#[cfg(test)]
pub fn assert_fold<As, Optic, S>(_o: Optic)
where
    Optic: Fold<As, S>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold() {
        let test: Vec<String> = vec!["Foo".into()];

        assert_fold::<AsFold, _, _, _>(Vec::<String>::into_iter);

        let mut iter = IntoIterator::into_iter.fold(test);
        assert!(iter.next().unwrap() == "Foo".to_string());
    }
}
