use crate::{Maybe, MaybeCell, NotVoid};

pub trait StaticMaybe<T>: Maybe<T>
where
    T: ?Sized
{
    const IS_SOME: bool;
    const IS_NONE: bool;
    type Some: StaticMaybe<T> + ?Sized;
    type None: StaticMaybe<T>;
    type Opposite: StaticMaybe<T> + ?Sized;
    type Maybe<M>: StaticMaybe<M> + ?Sized
    where
        M: StaticMaybe<M> + ?Sized;
    type MaybeOr<M, O>: ?Sized
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T,
        T: Sized;
        
    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>;

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(T,)>,
        F::Output: StaticMaybe<F::Output>,
        T: Sized,
        Self::Maybe<F::Output>: Sized;
}
impl<Some> const StaticMaybe<Some> for Some
where
    Some: NotVoid + ?Sized
{
    const IS_SOME: bool = true;
    const IS_NONE: bool = false;
    type None = ();
    type Some = Some;
    type Opposite = ();
    type Maybe<M> = M::Some
    where
        M: StaticMaybe<M> + ?Sized;
    type MaybeOr<M, O> = M
    where
        M: ?Sized,
        O: ?Sized;
    
    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        func()
    }
    
    fn maybe_or_from_fn<M, O>(maybe: M, _or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        maybe()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| map(self))
    }
}
impl<Some> const StaticMaybe<Some> for ()
where
    Some: NotVoid + ?Sized
{
    const IS_SOME: bool = false;
    const IS_NONE: bool = true;
    type None = ();
    type Some = Some;
    type Opposite = Some;
    type Maybe<M> = M::None
    where
        M: StaticMaybe<M> + ?Sized;
    type MaybeOr<M, O> = O
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(_func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        
    }

    fn maybe_or_from_fn<M, O>(_maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        or()
    }

    fn maybe_map<F>(self, _: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| panic!())
    }
}
impl const StaticMaybe<()> for ()
{
    const IS_SOME: bool = false;
    const IS_NONE: bool = true;
    type None = ();
    type Some = ();
    type Opposite = ();
    type Maybe<M> = M::None
    where
        M: StaticMaybe<M> + ?Sized;
    type MaybeOr<M, O> = O
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(_func: F) -> Self
    where
        F: FnOnce() -> (),
        (): Sized
    {
        
    }
    
    fn maybe_or_from_fn<M, O>(_maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        or()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<((),)>,
        F::Output: StaticMaybe<F::Output>,
        (): Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| map(self))
    }
}
impl<Some> const StaticMaybe<Some> for MaybeCell<Some, true>
where
    Some: StaticMaybe<Some>
{
    const IS_SOME: bool = true;
    const IS_NONE: bool = false;
    type None = Some::None;
    type Some = Some::Some;
    type Opposite = Some::None;
    type Maybe<M> = M::Some
    where
        M: StaticMaybe<M> + ?Sized;
    type MaybeOr<M, O> = M
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        Self::from_fn(func)
    }
    
    fn maybe_or_from_fn<M, O>(maybe: M, _or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        maybe()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| map(self.into_value()))
    }
}
impl<Some> const StaticMaybe<Some> for MaybeCell<Some, false>
where
    Some: StaticMaybe<Some>
{
    const IS_SOME: bool = false;
    const IS_NONE: bool = true;
    type None = Some::None;
    type Some = Some::Some;
    type Opposite = Some::Some;
    type Maybe<M> = M::None
    where
        M: StaticMaybe<M> + ?Sized;
    type MaybeOr<M, O> = O
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        Self::from_fn(func)
    }
    
    fn maybe_or_from_fn<M, O>(_maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        or()
    }

    fn maybe_map<F>(self, _: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| panic!())
    }
}