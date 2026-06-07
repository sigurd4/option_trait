use crate::{NotVoid, PureMaybe};

pub trait MaybeOr<T, Rhs>: PureMaybe<T>
where
    T: ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    type Output: PureMaybe<T> + ?Sized;

    fn or(maybe: Self, other: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized;

    fn or_else<F>(maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> Rhs,
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized;
}

impl<Lhs, Rhs, T> MaybeOr<T, Rhs> for Lhs
where
    T: ?Sized,
    Lhs: PureMaybe<T> + ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    default type Output = T;

    default fn or(_maybe: Self, _: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized
    {
        unreachable!()
    }

    default fn or_else<F>(_maybe: Self, _: F) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized
    {
        unreachable!()
    }
}

impl<T> MaybeOr<T, Option<T>> for Option<T>
{
    type Output = Option<T>;

    fn or(maybe: Self, other: Option<T>) -> Self::Output
    {
        maybe.or(other)
    }

    fn or_else<F>(maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> Option<T>
    {
        maybe.or_else(or_else)
    }
}
impl<T> MaybeOr<T, ()> for Option<T>
where
    T: NotVoid
{
    type Output = Option<T>;

    fn or(maybe: Self, (): ()) -> Self::Output
    {
        maybe
    }

    fn or_else<F>(maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce()
    {
        if maybe.is_none()
        {
            or_else()
        }
        maybe
    }
}
impl<T> MaybeOr<T, T> for Option<T>
{
    type Output = T;

    fn or(maybe: Self, rhs: T) -> Self::Output
    {
        maybe.unwrap_or(rhs)
    }

    fn or_else<F>(maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> T
    {
        maybe.unwrap_or_else(or_else)
    }
}

impl<T> MaybeOr<T, Option<T>> for ()
where
    T: NotVoid
{
    type Output = Option<T>;

    fn or(_maybe: Self, other: Option<T>) -> Self::Output
    {
        other
    }

    fn or_else<F>(_maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> Option<T>
    {
        or_else()
    }
}
impl<T> MaybeOr<T, ()> for ()
where
    T: NotVoid + ?Sized
{
    type Output = ();

    fn or(_maybe: Self, (): ()) -> Self::Output
    {
        
    }

    fn or_else<F>(_maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce()
    {
        or_else()
    }
}
impl<T> MaybeOr<T, T> for ()
where
    T: NotVoid + ?Sized
{
    type Output = T;

    fn or(_maybe: Self, rhs: T) -> Self::Output
    where
        Self::Output: Sized
    {
        rhs
    }

    fn or_else<F>(_maybe: Self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> T,
        Self::Output: Sized
    {
        or_else()
    }
}

impl<T, Rhs> MaybeOr<T, Rhs> for T
where
    T: ?Sized,
    Rhs: PureMaybe<T>
{
    type Output = T;

    fn or(maybe: Self, _: Rhs) -> Self::Output
    where
        Self::Output: Sized
    {
        maybe
    }

    fn or_else<F>(maybe: Self, _: F) -> Self::Output
    where
        Self::Output: Sized
    {
        maybe
    }
}