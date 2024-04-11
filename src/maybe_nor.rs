use crate::{NotVoid, StaticMaybe};

pub trait MaybeNor<T, Rhs>: StaticMaybe<T>
where
    T: ?Sized,
    Rhs: StaticMaybe<T> + ?Sized
{
    type Output: StaticMaybe<T> + ?Sized;
}

impl<T> MaybeNor<T, T> for T
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeNor<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeNor<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeNor<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;
}