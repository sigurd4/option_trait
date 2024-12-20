use crate::{NotVoid, PureMaybe};

pub trait MaybeAndThen<T, U, Rhs>: PureMaybe<T>
where
    T: ?Sized,
    U: ?Sized,
    Rhs: PureMaybe<U> + ?Sized
{
    type Output: PureMaybe<U> + ?Sized;

    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        T: Sized,
        F: FnOnce(T) -> Rhs,
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized;
}

impl<Lhs, Rhs, T, U> MaybeAndThen<T, U, Rhs> for Lhs
where
    T: ?Sized,
    U: ?Sized,
    Lhs: PureMaybe<T> + ?Sized,
    Rhs: PureMaybe<U> + ?Sized
{
    default type Output = U;

    default fn and_then<F>(self, _: F) -> Self::Output
    where
        T: Sized,
        F: FnOnce(T) -> Rhs,
        Self::Output: Sized,
        Self: Sized,
        Rhs: Sized
    {
        unreachable!()
    }
}

impl<T, U> MaybeAndThen<T, U, Option<U>> for Option<T>
{
    type Output = Option<U>;

    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        T: Sized,
        F: FnOnce(T) -> Option<U>
    {
        self.and_then(and_then)
    }
}
impl<T, U> MaybeAndThen<T, U, ()> for Option<T>
where
    U: NotVoid
{
    type Output = ();

    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        F: FnOnce(T) -> ()
    {
        if let Some(x) = self
        {
            and_then(x)
        }
    }
}
impl<T, U> MaybeAndThen<T, U, U> for Option<T>
{
    type Output = Option<U>;
    
    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        F: FnOnce(T) -> U
    {
        self.map(and_then)
    }
}

impl<Rhs, T, U> MaybeAndThen<T, U, Rhs> for ()
where
    T: ?Sized + NotVoid,
    U: ?Sized + NotVoid,
    Rhs: PureMaybe<U>
{
    type Output = ();

    fn and_then<F>(self, _: F) -> Self::Output
    {
        
    }
}
impl<Rhs, T> MaybeAndThen<T, (), Rhs> for ()
where
    T: ?Sized + NotVoid,
    Rhs: PureMaybe<()>
{
    type Output = ();

    fn and_then<F>(self, _: F) -> Self::Output
    {
        
    }
}

impl<T, U> MaybeAndThen<T, U, Option<U>> for T
{
    type Output = Option<U>;

    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        F: FnOnce(T) -> Option<U>
    {
        and_then(self)
    }
}
impl<T, U> MaybeAndThen<T, U, ()> for T
where
    T: ?Sized,
    U: NotVoid + ?Sized,
{
    type Output = ();

    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        T: Sized,
        F: FnOnce(T) -> ()
    {
        and_then(self)
    }
}
impl<T, U> MaybeAndThen<T, U, U> for T
where
    U: ?Sized,
    T: ?Sized
{
    type Output = U;

    fn and_then<F>(self, and_then: F) -> Self::Output
    where
        T: Sized,
        F: FnOnce(T) -> U,
        Self::Output: Sized,
        Self: Sized
    {
        and_then(self)
    }
}