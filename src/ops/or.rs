use crate::{NotVoid, PureMaybe};

pub trait MaybeOr<T, Rhs>: PureMaybe<T>
where
    T: ?Sized,
    Rhs: PureMaybe<T> + ?Sized
{
    type Output: PureMaybe<T> + ?Sized;

    fn or(self, other: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized;

    fn or_else<F>(self, or_else: F) -> Self::Output
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

    default fn or(self, _: Rhs) -> Self::Output
    where
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized
    {
        unreachable!()
    }

    default fn or_else<F>(self, _: F) -> Self::Output
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

    fn or(self, other: Option<T>) -> Self::Output
    {
        self.or(other)
    }

    fn or_else<F>(self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> Option<T>
    {
        self.or_else(or_else)
    }
}
impl<T> MaybeOr<T, ()> for Option<T>
where
    T: NotVoid
{
    type Output = Option<T>;

    fn or(self, (): ()) -> Self::Output
    {
        self
    }

    fn or_else<F>(self, or_else: F) -> Self::Output
    where
        F: FnOnce()
    {
        if self.is_none()
        {
            or_else()
        }
        self
    }
}
impl<T> MaybeOr<T, T> for Option<T>
{
    type Output = T;

    fn or(self, rhs: T) -> Self::Output
    {
        self.unwrap_or(rhs)
    }

    fn or_else<F>(self, or_else: F) -> Self::Output
    where
        F: FnOnce() -> T
    {
        self.unwrap_or_else(or_else)
    }
}

impl<T> MaybeOr<T, Option<T>> for ()
where
    T: NotVoid
{
    type Output = Option<T>;

    fn or(self, other: Option<T>) -> Self::Output
    {
        other
    }

    fn or_else<F>(self, or_else: F) -> Self::Output
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

    fn or(self, (): ()) -> Self::Output
    {
        
    }

    fn or_else<F>(self, or_else: F) -> Self::Output
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

    fn or(self, rhs: T) -> Self::Output
    where
        Self::Output: Sized
    {
        rhs
    }

    fn or_else<F>(self, or_else: F) -> Self::Output
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

    fn or(self, _: Rhs) -> Self::Output
    where
        Self::Output: Sized
    {
        self
    }

    fn or_else<F>(self, _: F) -> Self::Output
    where
        Self::Output: Sized
    {
        self
    }
}