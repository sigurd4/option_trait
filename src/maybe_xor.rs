use crate::{private::NotVoid, StaticMaybe};

pub trait MaybeXor<T, Rhs>: StaticMaybe<T>
where
    Rhs: StaticMaybe<T>
{
    type Output: StaticMaybe<T>;
}

impl<T> MaybeXor<T, T> for T
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeXor<T, ()> for T
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeXor<T, T> for ()
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeXor<T, ()> for ()
where
    T: NotVoid
{
    type Output = ();
}