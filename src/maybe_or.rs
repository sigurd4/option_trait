use crate::{NotVoid, StaticMaybe};

pub trait MaybeOr<T, Rhs>: StaticMaybe<T>
where
    Rhs: StaticMaybe<T>
{
    type Output: StaticMaybe<T>;
}

impl<T> MaybeOr<T, T> for T
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeOr<T, ()> for T
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeOr<T, T> for ()
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeOr<T, ()> for ()
where
    T: NotVoid
{
    type Output = ();
}