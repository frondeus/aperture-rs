use crate::{
    method::{Method, MethodOnce},
    sealed::Sealed,
};

pub trait LazyExt<S, A>: MethodOnce<S, A> + Sized {
    fn with_args(self, args: A) -> WithArgs<Self, A> {
        WithArgs { method: self, args }
    }
    fn lazy<F>(self, func: F) -> WithLazyArgs<Self, F> {
        WithLazyArgs { method: self, func }
    }
}

impl<M, S, A> LazyExt<S, A> for M where M: Sized + MethodOnce<S, A> {}

pub struct WithArgs<M, A> {
    method: M,
    args: A,
}

pub struct WithLazyArgs<M, F> {
    method: M,
    func: F,
}

impl<S, F, M> Sealed<S, ()> for WithArgs<M, F> {}
impl<S, F, M> Sealed<S, ()> for WithLazyArgs<M, F> {}

impl<M, S, A, F> MethodOnce<S, ()> for WithLazyArgs<M, F>
where
    M: MethodOnce<S, A>,
    F: FnOnce() -> A,
{
    type Output = M::Output;

    fn mcall_once(self, s: S, (): ()) -> Self::Output {
        self.method.mcall_once(s, (self.func)())
    }
}
impl<M, S, A, F> Method<S, ()> for WithLazyArgs<M, F>
where
    M: Method<S, A>,
    F: Fn() -> A,
{
    fn mcall(&self, s: S, (): ()) -> Self::Output {
        self.method.mcall(s, (self.func)())
    }
}

impl<M, S, A> MethodOnce<S, ()> for WithArgs<M, A>
where
    M: MethodOnce<S, A>,
{
    type Output = M::Output;

    fn mcall_once(self, s: S, (): ()) -> Self::Output {
        self.method.mcall_once(s, self.args)
    }
}

impl<M, S, A> Method<S, ()> for WithArgs<M, A>
where
    M: Method<S, A>,
    A: Copy,
{
    fn mcall(&self, s: S, (): ()) -> Self::Output {
        self.method.mcall(s, self.args)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Arg, Test};

    use super::*;

    #[test]
    fn test() {
        let mut test = Test("Foo".into());

        assert_eq!(Test::ref_.with_args(()).mcall_once(&test, ()), "Foo");
        assert_eq!(Test::mut_.with_args(()).mcall_once(&mut test, ()), "Foo");
        assert_eq!(Test::prop_.with_args(()).mcall_once(&test, ()), "Foo");
        assert_eq!(Test::own_.with_args(()).mcall_once(test, ()), "Foo");
        let mut test = Test("Foo".into());
        assert_eq!(Test::ref_arg.with_args((1,)).mcall_once(&test, ()), "Foo");
        assert_eq!(
            Test::mut_arg.with_args((1,)).mcall_once(&mut test, ()),
            "Foo"
        );
        assert_eq!(Test::prop_arg.with_args((1,)).mcall_once(&test, ()), "Foo");
        assert_eq!(Test::own_arg.with_args((1,)).mcall_once(test, ()), "Foo");

        let mut test = Test("Foo".into());
        assert_eq!(Test::ref_.with_args(()).mcall(&test, ()), "Foo");
        assert_eq!(Test::mut_.with_args(()).mcall(&mut test, ()), "Foo");
        assert_eq!(Test::prop_.with_args(()).mcall(&test, ()), "Foo");
        assert_eq!(Test::own_.with_args(()).mcall(test, ()), "Foo");

        let mut test = Test("Foo".into());
        assert_eq!(Test::ref_arg.with_args((1,)).mcall(&test, ()), "Foo");
        assert_eq!(Test::mut_arg.with_args((1,)).mcall(&mut test, ()), "Foo");
        assert_eq!(Test::prop_arg.with_args((1,)).mcall(&test, ()), "Foo");
        assert_eq!(Test::own_arg.with_args((1,)).mcall(test, ()), "Foo");

        let mut test = Test("Foo".into());
        assert_eq!(
            Test::ref_complex.with_args((Arg,)).mcall_once(&test, ()),
            "Foo"
        );
        assert_eq!(
            Test::mut_complex
                .with_args((Arg,))
                .mcall_once(&mut test, ()),
            "Foo"
        );
        assert_eq!(
            Test::prop_complex.with_args((Arg,)).mcall_once(&test, ()),
            "Foo"
        );
        assert_eq!(
            Test::own_complex.with_args((Arg,)).mcall_once(test, ()),
            "Foo"
        );
    }
}
