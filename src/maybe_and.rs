use crate::{NotVoid, StaticMaybe};

pub trait MaybeAnd<T, Rhs>: StaticMaybe<T>
where
    T: ?Sized,
    Rhs: StaticMaybe<T> + ?Sized
{
    type Output: StaticMaybe<T> + ?Sized;
}

impl<T> MaybeAnd<T, T> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeAnd<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeAnd<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeAnd<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();
}