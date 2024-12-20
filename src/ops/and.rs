use crate::{ops::MaybeAndThen, NotVoid, PureMaybe};

pub trait MaybeAnd<T, Rhs>: MaybeAndThen<T, T, Rhs>
where
    T: ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    type Output: PureMaybe<T> + ?Sized = <Self as MaybeAndThen<T, T, Rhs>>::Output;

    fn and(self, other: Rhs) -> <Self as MaybeAnd<T, Rhs>>::Output
    where
        <Self as MaybeAnd<T, Rhs>>::Output: Sized,
        Self: Sized,
        Rhs: Sized;
}

impl<Lhs, Rhs, T> MaybeAnd<T, Rhs> for Lhs
where
    T: ?Sized,
    Lhs: PureMaybe<T> + ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    default fn and(self, _: Rhs) -> <Self as MaybeAnd<T, Rhs>>::Output
    where
        <Self as MaybeAnd<T, Rhs>>::Output: Sized,
        Self: Sized,
        Rhs: Sized
    {
        unreachable!()
    }
}

impl<T> MaybeAnd<T, Option<T>> for Option<T>
{
    fn and(self, rhs: Option<T>) -> <Self as MaybeAnd<T, Option<T>>>::Output
    where
        <Self as MaybeAnd<T, Option<T>>>::Output: Sized
    {
        self.and(rhs)
    }
}
impl<T> MaybeAnd<T, ()> for Option<T>
where
    T: NotVoid
{
    fn and(self, (): ()) -> <Self as MaybeAnd<T, ()>>::Output
    {
        
    }
}
impl<T> MaybeAnd<T, T> for Option<T>
{
    fn and(self, rhs: T) -> <Self as MaybeAnd<T, T>>::Output
    {
        self.map(|_| rhs)
    }
}

impl<Rhs, T> MaybeAnd<T, Rhs> for ()
where
    T: ?Sized + NotVoid,
    Rhs: PureMaybe<T>
{
    fn and(self, _: Rhs) -> <Self as MaybeAnd<T, Rhs>>::Output
    where
        <Self as MaybeAnd<T, Rhs>>::Output: Sized
    {
        
    }
}

impl<T> MaybeAnd<T, Option<T>> for T
{
    fn and(self, other: Option<T>) -> <Self as MaybeAnd<T, Option<T>>>::Output
    where
        <Self as MaybeAnd<T, Option<T>>>::Output: Sized
    {
        other
    }
}
impl<T> MaybeAnd<T, ()> for T
where
    T: NotVoid + ?Sized
{
    fn and(self, (): ()) -> <Self as MaybeAnd<T, ()>>::Output
    where
        Self: Sized,
        <Self as MaybeAnd<T, ()>>::Output: Sized
    {
        
    }
}
impl<T> MaybeAnd<T, T> for T
where
    T: ?Sized
{
    fn and(self, rhs: T) -> <Self as MaybeAnd<T, T>>::Output
    where
        <Self as MaybeAnd<T, T>>::Output: Sized
    {
        rhs
    }
}