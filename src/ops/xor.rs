use crate::{Maybe, NotVoid, PureMaybe};

pub trait MaybeXor<T, Rhs>: PureMaybe<T>
where
    T: ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    type Output: PureMaybe<T> + ?Sized;

    fn xor(maybe: Self, other: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized;
}

impl<Lhs, Rhs, T> MaybeXor<T, Rhs> for Lhs
where
    T: ?Sized,
    Lhs: PureMaybe<T> + ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    default type Output = T;

    default fn xor(_maybe: Self, _: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized
    {
        unreachable!()
    }
}

impl<T> MaybeXor<T, Option<T>> for Option<T>
{
    type Output = Option<T>;

    fn xor(maybe: Self, other: Option<T>) -> Self::Output
    {
        maybe.xor(other)
    }
}
impl<T> MaybeXor<T, ()> for Option<T>
where
    T: NotVoid
{
    type Output = Option<T>;

    fn xor(maybe: Self, (): ()) -> Self::Output
    {
        maybe
    }
}
impl<T> MaybeXor<T, T> for Option<T>
{
    type Output = Option<T>;

    fn xor(maybe: Self, other: T) -> Self::Output
    {
        maybe.xor(Maybe::option(other))
    }
}

impl<Rhs, T> MaybeXor<T, Rhs> for ()
where
    Rhs: PureMaybe<T> + ?Sized,
    T: NotVoid + ?Sized
{
    type Output = Rhs;

    fn xor(_maybe: Self, other: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized
    {
        other
    }
}

impl<T> MaybeXor<T, Option<T>> for T
{
    type Output = Option<T>;

    fn xor(maybe: Self, other: Option<T>) -> Self::Output
    {
        Maybe::option(maybe).xor(other)
    }
}
impl<T> MaybeXor<T, ()> for T
where
    T: NotVoid + ?Sized
{
    type Output = T;

    fn xor(maybe: Self, (): ()) -> Self::Output
    where
        Self: Sized
    {
        maybe
    }
}
impl<T> MaybeXor<T, T> for T
where
    T: ?Sized,
    (): PureMaybe<T>
{
    type Output = ();

    fn xor(_maybe: Self, _: T) -> Self::Output
    where
        Self: Sized
    {
        
    }
}