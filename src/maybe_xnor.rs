use crate::{private::NotVoid, StaticMaybe};

pub trait MaybeXnor<T, Rhs>: StaticMaybe<T>
where
    Rhs: StaticMaybe<T>
{
    type Output: StaticMaybe<T>;
}

impl<T> MaybeXnor<T, T> for T
where
    T: NotVoid
{
    type Output = T;
}
impl<T> MaybeXnor<T, ()> for T
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeXnor<T, T> for ()
where
    T: NotVoid
{
    type Output = ();
}
impl<T> MaybeXnor<T, ()> for ()
where
    T: NotVoid
{
    type Output = T;
}