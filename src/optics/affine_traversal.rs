use super::{AffineFold, Fold, Setter, Traversal};
use crate::method::Method;

pub struct AsAffineTraversal;
pub trait AffineTraversal<As, S, Out, F>
where
    Self: Traversal<As, S, Out, F> + AffineFold<As, S>,
    F: FnMut(<Self as Fold<As, S>>::T) -> Out,
{
    fn map_opt(&self, source: S, f: F) -> Option<Out>;
}

impl<S, M, In, Out, F> AffineTraversal<AsAffineTraversal, S, Out, F> for M
where
    M: Method<S, (), Output = Option<In>>
        + Traversal<AsAffineTraversal, S, Out, F>
        + AffineFold<AsAffineTraversal, S>
        + Setter<AsAffineTraversal, S, In = In>
        + Fold<AsAffineTraversal, S, T = In>,

    F: FnMut(In) -> Out,
{
    fn map_opt(&self, source: S, f: F) -> Option<Out> {
        self.mcall(source, ()).map(f)
    }
}

impl<S, M, In, Out, F> Traversal<AsAffineTraversal, S, Out, F> for M
where
    M: Method<S, (), Output = Option<In>>,
    F: FnMut(In) -> Out,
{
    type TraversalIter = std::iter::Map<std::option::IntoIter<In>, F>;

    fn map(&self, source: S, f: F) -> Self::TraversalIter {
        self.mcall(source, ()).into_iter().map(f)
    }
}

impl<S, M, In> AffineFold<AsAffineTraversal, S> for M
where
    M: Method<S, (), Output = Option<In>> + Fold<AsAffineTraversal, S, T = In>,
{
    fn preview(&self, source: S) -> Option<Self::T> {
        self.mcall(source, ())
    }
}

impl<S, M, In> Setter<AsAffineTraversal, S> for M
where
    M: Method<S, (), Output = Option<In>>,
{
    type In = In;

    fn set<F>(&self, mut source: S, f: F) -> S
    where
        F: FnOnce(&mut In),
    {
        // self.mcall(source, ()).map(f)
        todo!()
    }
}

impl<S, M, In> Fold<AsAffineTraversal, S> for M
where
    M: Method<S, (), Output = Option<In>>,
{
    type T = In;

    type FoldIter = std::option::IntoIter<In>;

    fn fold(&self, source: S) -> Self::FoldIter {
        self.mcall(source, ()).into_iter()
    }
}
// #[cfg(test)]
// pub fn assert_affine_traversal<As, Optic, S, Out>(_o: Optic)
// where
//     Optic: AffineTraversal<As, S, Out, crate::identity::Identity, In = Out>,
// {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity;

    #[test]
    fn affine_traversal() {
        let test: Option<String> = Some("Foo".into());

        assert_eq!(
            Option::as_ref
                .map_opt(&test, |x| x.to_uppercase())
                .expect("some"),
            "FOO"
        );

        let test: Option<String> = Some("Foo".into());
        assert_eq!(
            Identity::option::<String>
                .map_opt(test, |x| x.to_uppercase())
                .expect("some"),
            "FOO"
        );
        // let mut map: HashMap<usize, String> = HashMap::new();
        // map.insert(1, "Foo".into());

        // let test: Option<String> = Some("Foo".into());
        // assert_eq!(
        //     HashMap::get.with_args((&1,)).preview(&map).expect("some"),
        //     "Foo"
        // );
    }

    #[test]
    fn as_traversal() {
        // assert_traversal(Option::<String>::as_ref);

        let test: Option<String> = Some("Foo".into());
        assert_eq!(
            Option::as_ref
                .map(&test, |x| x.to_uppercase())
                .next()
                .expect("some"),
            "FOO"
        );
    }
}
