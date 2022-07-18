use crate::sealed::Sealed;

pub trait MethodOnce<S, A>: Sealed<S, A> {
    type Output;

    fn mcall_once(self, s: S, args: A) -> Self::Output;
}

pub trait Method<S, A>: MethodOnce<S, A> + Sealed<S, A> {
    fn mcall(&self, s: S, args: A) -> Self::Output;
}

macro_rules! impl_method {
    ($one: ident) => {
        impl_method!(@ $one);
        impl_method!(@);
    };
    ($first: ident, $($args: ident),*) => {
        impl_method!(@ $first $($args)*);
        impl_method!($($args),*);
    };
    (@ $($arg: ident)*) => {
        impl<F, S, V, $($arg),*> Method<S, ($($arg,)*)> for F
        where F: Fn(S, $($arg),*) -> V
        {
            #[allow(non_snake_case)]
            fn mcall(&self, r: S, ($($arg,)*): ($($arg,)*)) -> Self::Output {
                self(r, $($arg),*)
            }
        }
        impl<F, S, V, $($arg),*> Sealed<S, ($($arg,)*)> for F where F: FnOnce(S, $($arg),*) -> V { }
        impl<F, S, V, $($arg),*> MethodOnce<S, ($($arg,)*)> for F
        where F: FnOnce(S, $($arg),*) -> V
        {
            type Output = V;
            #[allow(non_snake_case)]
            fn mcall_once(self, r: S, ($($arg,)*): ($($arg,)*)) -> Self::Output {
                self(r, $($arg),*)
            }
        }
    };
}

impl_method! {A1, A2, A3, A4}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Test;

    #[test]
    fn test() {
        let mut test = Test("Foo".into());

        assert_eq!(Test::ref_.mcall(&test, ()), "Foo");
        assert_eq!(Test::mut_.mcall(&mut test, ()), "Foo");
        assert_eq!(Test::prop_.mcall(&test, ()), "Foo");
        assert_eq!(Test::own_.mcall(test, ()), "Foo");

        let mut test = Test("Foo".into());
        assert_eq!(Test::ref_arg.mcall(&test, (1,)), "Foo");
        assert_eq!(Test::mut_arg.mcall(&mut test, (1,)), "Foo");
        assert_eq!(Test::prop_arg.mcall(&test, (1,)), "Foo");
        assert_eq!(Test::own_arg.mcall(test, (1,)), "Foo");
    }
}
