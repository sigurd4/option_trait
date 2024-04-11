use crate::{NotVoid, StaticMaybe};

pub trait MaybeXor<T, Rhs>: StaticMaybe<T>
where
    T: ?Sized,
    Rhs: StaticMaybe<T> + ?Sized
{
    type Output: StaticMaybe<T> + ?Sized;
}

impl<T> MaybeXor<T, T> for T
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeXor<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeXor<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeXor<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();
}