use super::fold::FoldLike;

pub trait AffineFoldLike<S, Marker> {
    type T;
    fn preview(&self, source: S) -> Option<Self::T>;
}

pub struct IsAffineFold;

impl<S, M, T> FoldLike<S, (IsAffineFold, M)> for T
where
    T: AffineFoldLike<S, M>,
{
    type T = <Self as AffineFoldLike<S, M>>::T;

    type Iter = std::option::IntoIter<<Self as AffineFoldLike<S, M>>::T>;

    fn fold(&self, source: S) -> Self::Iter {
        self.preview(source).into_iter()
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::lazy::LazyExt;

    use super::*;

    #[test]
    fn optional() {
        let test: Option<String> = Some("Foo".into());
        assert_eq!(Option::as_ref.preview(&test).expect("some"), "Foo");

        let mut map: HashMap<usize, String> = HashMap::new();
        map.insert(1, "Foo".into());

        let test: Option<String> = Some("Foo".into());
        assert_eq!(
            HashMap::get.with_args((&1,)).preview(&map).expect("some"),
            "Foo"
        );
        assert_eq!(HashMap::get.with_args((&2,)).preview(&map), None);

        let test: Option<String> = Some("Foo".into());

        assert_eq!(Option::as_ref.fold(&test).next().expect("some"), "Foo");

        // let vec: Vec<String> = vec!["Foo".into()];
        // assert_eq!(<>::get.lazy((0,)).preview(&vec), None);
    }
}
