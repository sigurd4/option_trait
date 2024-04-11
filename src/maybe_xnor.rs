use crate::{NotVoid, StaticMaybe};

pub trait MaybeXnor<T, Rhs>: StaticMaybe<T>
where
    T: ?Sized,
    Rhs: StaticMaybe<T> + ?Sized
{
    type Output: StaticMaybe<T> + ?Sized;
}

impl<T> MaybeXnor<T, T> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;
}
impl<T> MaybeXnor<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeXnor<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();
}
impl<T> MaybeXnor<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;
}