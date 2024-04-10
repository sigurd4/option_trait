use crate::{NotVoid, StaticMaybe};

pub trait MaybeNand<T, Rhs>: StaticMaybe<T>
where
    Rhs: StaticMaybe<T>
{
    type Output: StaticMaybe<T>;
}

impl<T> MaybeNand<T, T> for T
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeNand<T, ()> for T
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeNand<T, T> for ()
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeNand<T, ()> for ()
where
    T: NotVoid
{
    type Output = T;
}