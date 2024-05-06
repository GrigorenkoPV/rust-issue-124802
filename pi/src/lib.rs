use std::marker::PhantomData;

/// PI stands for ParallelIterator
pub trait PI: Sized + Send {
    type Item: Send;
}

pub trait IntoPI {
    type I: PI;
    fn into_pi(self) -> Self::I;
}

impl<T: PI> IntoPI for T {
    type I = T;
    fn into_pi(self) -> Self::I {
        self
    }
}

impl<T: Send> IntoPI for Option<T> {
    type I = Iter<T>;

    fn into_pi(self) -> Self::I {
        Iter(PhantomData)
    }
}

pub struct Iter<T>(PhantomData<T>);

impl<T: Send> PI for Iter<T> {
    type Item = T;
}
