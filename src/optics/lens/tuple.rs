use super::*;

impl<'a, G, S, T, Src, GM> GetLike<'a, Src, (IsLens, GM)> for (G, S, T)
where
    G: GetLike<'a, Src, GM>,
    Src: 'a,
{
    type T = G::T;

    fn view(&self, source: &'a Src) -> &'a Self::T {
        self.0.view(source)
    }
}
impl<'a, G, S, T, Src, SM> SetLike<'a, Src, (IsLens, SM)> for (G, S, T)
where
    S: SetLike<'a, Src, SM>,
    Src: 'a,
{
    type T = S::T;

    fn set<F>(&self, source: &'a mut Src, f: F)
    where
        F: FnOnce(&'a mut Self::T),
    {
        self.1.set(source, f)
    }
}
// impl<'a, G, S, T, Src, TM> AffineFoldLike<'a, Src, (IsLens, TM)> for (G, S, T)
// where
//     T: AffineFoldLike<'a, Src, TM>,
//     Src: 'a,
// {
//     type T = T::T;

//     fn preview(&self, source: &'a Src) -> Option<&'a Self::T> {
//         self.2.preview(source)
//     }
// }
