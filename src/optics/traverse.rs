pub trait TraversalLike<'a, S, Marker> {
    type T: 'a;
    fn preview(&self, source: &'a S) -> Option<&'a Self::T>;
}
pub trait TraversalLikeOnce<'a, S> {
    type T: 'a;
    fn preview(self, source: &'a S) -> Option<&'a Self::T>;
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::lazy::LazyExt;

    use super::*;

    #[test]
    fn traversal() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(Option::as_ref.preview(&test).expect("some"), "Foo");

        let mut map: HashMap<usize, String> = HashMap::new();
        map.insert(1, "Foo".into());

        assert_eq!(
            HashMap::get.with_args((&1,)).preview(&map).expect("some"),
            "Foo"
        );
        assert_eq!(HashMap::get.with_args((&2,)).preview(&map), None);

        // let vec: Vec<String> = vec!["Foo".into()];
        // assert_eq!(<>::get.lazy((0,)).preview(&vec), None);
    }
}
