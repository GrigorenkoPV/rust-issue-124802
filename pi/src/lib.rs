pub trait PI: Sized + Send {
    type Item: Send;
}

pub trait IntoPI {
    type I: PI;
    fn into_pi(self) -> Self::I;
}

impl<T: PI> IntoPI for T {
    type I = T;
    fn into_pi(self) -> T {
        self
    }
}

impl<'a, T> IntoPI for &'a [T]
where
    T: Sync + 'a,
{
    type I = ();

    fn into_pi(self) -> Self::I {}
}

impl<'a, T> IntoPI for &'a mut [T]
where
    T: Send + 'a,
{
    type I = ();

    fn into_pi(self) -> Self::I {}
}

impl<'a, T> IntoPI for &'a Vec<T>
where
    T: Sync + 'a,
{
    type I = ();

    fn into_pi(self) -> Self::I {}
}

impl<'a, T> IntoPI for &'a mut Vec<T>
where
    T: Send + 'a,
{
    type I = ();

    fn into_pi(self) -> Self::I {}
}

impl PI for () {
    type Item = ();
}

#[derive(Debug, Clone)]
pub struct VecIter<T: Send>(pub Vec<T>);

impl<T: Send> IntoPI for Vec<T> {
    type I = VecIter<T>;

    fn into_pi(self) -> Self::I {
        VecIter(self)
    }
}

impl<T: Send> PI for VecIter<T> {
    type Item = T;
}
