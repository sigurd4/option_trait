use crate::{NotVoid, StaticMaybe};

pub trait MaybeOr<T, Rhs>: StaticMaybe<T>
where
    T: ?Sized,
    Rhs: StaticMaybe<T> + ?Sized
{
    type Output: StaticMaybe<T> + ?Sized;
}

impl<T> MaybeOr<T, T> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeOr<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeOr<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeOr<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();
}