use crate::{private, Maybe, MaybeCell};

pub trait StaticMaybe<T>: Maybe<T>
{
    type Opposite: StaticMaybe<T>;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T;
}
impl<Some> const StaticMaybe<Some> for Some
where
    Some: private::NotVoid
{
    type Opposite = ();
    
    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some
    {
        func()
    }
}
impl<Some> const StaticMaybe<Some> for ()
where
    Some: private::NotVoid
{
    type Opposite = Some;

    fn maybe_from_fn<F>(_func: F) -> Self
    where
        F: FnOnce() -> Some
    {
        ()
    }
}
impl<Some> const StaticMaybe<Some> for MaybeCell<Some, true>
where
    Some: private::NotVoid,
{
    type Opposite = ();

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some
    {
        Self::from_fn(func)
    }
}
impl<Some> const StaticMaybe<Some> for MaybeCell<Some, false>
where
    Some: private::NotVoid,
{
    type Opposite = Some;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some
    {
        Self::from_fn(func)
    }
}