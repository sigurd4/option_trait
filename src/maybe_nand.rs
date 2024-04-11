use crate::{NotVoid, StaticMaybe};

pub trait MaybeNand<T, Rhs>: StaticMaybe<T>
where
    T: ?Sized,
    Rhs: StaticMaybe<T> + ?Sized
{
    type Output: StaticMaybe<T> + ?Sized;
}

impl<T> MaybeNand<T, T> for T
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeNand<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeNand<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeNand<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;
}