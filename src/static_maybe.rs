use crate::{Maybe, MaybeCell, NotVoid};

pub trait StaticMaybe<T>: Maybe<T>
where
    T: ?Sized
{
    type Some: StaticMaybe<T> + ?Sized;
    type None: StaticMaybe<T>;
    type Opposite: StaticMaybe<T> + ?Sized;
    type Maybe<M>: StaticMaybe<M> + ?Sized
    where
        M: StaticMaybe<M> + ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T,
        T: Sized;
}
impl<Some> const StaticMaybe<Some> for Some
where
    Some: NotVoid + ?Sized
{
    type None = ();
    type Some = Some;
    type Opposite = ();
    type Maybe<M> = M::Some
    where
        M: StaticMaybe<M> + ?Sized;
    
    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        func()
    }
}
impl<Some> const StaticMaybe<Some> for ()
where
    Some: NotVoid + ?Sized
{
    type None = ();
    type Some = Some;
    type Opposite = Some;
    type Maybe<M> = M::None
    where
        M: StaticMaybe<M> + ?Sized;

    fn maybe_from_fn<F>(_func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        ()
    }
}
impl<Some> const StaticMaybe<Some> for MaybeCell<Some, true>
where
    Some: NotVoid,
{
    type None = ();
    type Some = Some;
    type Opposite = ();
    type Maybe<M> = M::Some
    where
        M: StaticMaybe<M> + ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        Self::from_fn(func)
    }
}
impl<Some> const StaticMaybe<Some> for MaybeCell<Some, false>
where
    Some: NotVoid,
{
    type None = ();
    type Some = Some;
    type Opposite = Some;
    type Maybe<M> = M::None
    where
        M: StaticMaybe<M> + ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        Self::from_fn(func)
    }
}