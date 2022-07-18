pub struct Identity;
impl<T> FnOnce<(T,)> for Identity {
    type Output = T;
    extern "rust-call" fn call_once(self, (t,): (T,)) -> Self::Output {
        t
    }
}
impl<T> Fn<(T,)> for Identity {
    extern "rust-call" fn call(&self, (t,): (T,)) -> Self::Output {
        t
    }
}
impl<T> FnMut<(T,)> for Identity {
    extern "rust-call" fn call_mut(&mut self, (t,): (T,)) -> Self::Output {
        t
    }
}
