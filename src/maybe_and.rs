use crate::{NotVoid, StaticMaybe};

pub trait MaybeAnd<T, Rhs>: StaticMaybe<T>
where
    Rhs: StaticMaybe<T>
{
    type Output: StaticMaybe<T>;
}

impl<T> MaybeAnd<T, T> for T
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeAnd<T, ()> for T
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeAnd<T, T> for ()
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeAnd<T, ()> for ()
where
    T: NotVoid
{
    type Output = ();
}