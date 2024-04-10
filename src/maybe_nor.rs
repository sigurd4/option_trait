use crate::{NotVoid, StaticMaybe};

pub trait MaybeNor<T, Rhs>: StaticMaybe<T>
where
    Rhs: StaticMaybe<T>
{
    type Output: StaticMaybe<T>;
}

impl<T> MaybeNor<T, T> for T
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeNor<T, ()> for T
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeNor<T, T> for ()
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeNor<T, ()> for ()
where
    T: NotVoid
{
    type Output = T;
}