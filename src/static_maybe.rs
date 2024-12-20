use crate::{Maybe, MaybeCell, NotVoid};

pub trait StaticMaybe<T>: Maybe<T>
where
    T: ?Sized
{
    const IS_SOME: bool;
    const IS_NONE: bool;
    type Some: StaticMaybe<T> + ?Sized;
    type None: StaticMaybe<T>
    where
        (): StaticMaybe<T>;
    type Opposite: StaticMaybe<T> + ?Sized
    where
        (): StaticMaybe<T>;
    type Maybe<M>: StaticMaybe<M> + ?Sized
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
    type MaybeOr<M, O>: ?Sized
    where
        M: ?Sized,
        O: ?Sized;

    /// Constructs a new maybe from the given function, if the type can contain a value.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// fn f() -> &'static str
    /// {
    ///     "ok"
    /// }
    /// 
    /// assert_eq!(<&str as StaticMaybe<&str>>::maybe_from_fn(f), "ok");
    /// assert_eq!(<() as StaticMaybe<&str>>::maybe_from_fn(f), ());
    /// assert_eq!(<[&str; 1] as StaticMaybe<&str>>::maybe_from_fn(f), ["ok"]);
    /// assert_eq!(<[&str; 0] as StaticMaybe<&str>>::maybe_from_fn(f), [] as [&str; 0]);
    /// ```
    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T,
        T: Sized;
        
    /// Either runs `maybe` or `or` depending on wether or not the maybe can contain a value.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// fn maybe() -> &'static str
    /// {
    ///     "ok"
    /// }
    /// 
    /// fn or() -> bool
    /// {
    ///     false
    /// }
    /// 
    /// assert_eq!(<&str as StaticMaybe<&str>>::maybe_or_from_fn(maybe, or), "ok");
    /// assert_eq!(<() as StaticMaybe<&str>>::maybe_or_from_fn(maybe, or), false);
    /// assert_eq!(<[&str; 1] as StaticMaybe<&str>>::maybe_or_from_fn(maybe, or), "ok");
    /// assert_eq!(<[&str; 0] as StaticMaybe<&str>>::maybe_or_from_fn(maybe, or), false);
    /// ```
    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>;

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(T,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        T: Sized,
        Self::Maybe<F::Output>: Sized;

    fn maybe(self) -> Self::Maybe<T>
    where
        T: Sized + StaticMaybe<T>,
        (): StaticMaybe<T>,
        Self::Maybe<T>: Sized;

    fn into_value(self) -> T
    where
        Self: StaticMaybe<T, Maybe<T> = T>,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self: Sized
    {
        self.unwrap()
    }
}
impl<Some> /*const*/ StaticMaybe<Some> for Some
where
    Some: ?Sized
{
    const IS_SOME: bool = true;
    const IS_NONE: bool = false;
    type None = ()
    where
        (): StaticMaybe<Some>;
    type Some = Some;
    type Opposite = ()
    where
        (): StaticMaybe<Some>;
    type Maybe<M> = M
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
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
    
    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        core::mem::drop(or);
        maybe()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| map(self))
    }
    fn maybe(self) -> Self::Maybe<Some>
    where
        Some: Sized,
        (): StaticMaybe<Some>
    {
        self
    }
}
impl<Some> /*const*/ StaticMaybe<Some> for ()
where
    Some: NotVoid + ?Sized
{
    const IS_SOME: bool = false;
    const IS_NONE: bool = true;
    type None = ();
    type Some = Some;
    type Opposite = Some;
    type Maybe<M> = ()
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
    type MaybeOr<M, O> = O
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        core::mem::drop(func);
    }

    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        core::mem::drop(maybe);
        or()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        core::mem::drop(map);
        StaticMaybe::maybe_from_fn(|| panic!())
    }
    fn maybe(self) -> Self::Maybe<Some>
    where
        Some: Sized + StaticMaybe<Some>,
        Self::Maybe<Some>: Sized
    {
        
    }
}
impl<Some> /*const*/ StaticMaybe<Some> for [Some; 0]
where
    Some: StaticMaybe<Some>
{
    const IS_SOME: bool = false;
    const IS_NONE: bool = true;
    type None = [Some; 0]
    where
        (): StaticMaybe<Some>;
    type Some = [Some; 1];
    type Opposite = Self::Some
    where
        (): StaticMaybe<Some>;
    type Maybe<M> = ()
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
    type MaybeOr<M, O> = O
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        core::mem::drop(func);
        []
    }

    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        core::mem::drop(maybe);
        or()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        core::mem::drop(map);
        StaticMaybe::maybe_from_fn(|| panic!())
    }
    fn maybe(self) -> Self::Maybe<Some>
    where
        Some: Sized + StaticMaybe<Some>,
        (): StaticMaybe<Some>,
        Self::Maybe<Some>: Sized
    {
        
    }
}
impl<Some> /*const*/ StaticMaybe<Some> for [Some; 1]
where
    Some: StaticMaybe<Some>
{
    const IS_SOME: bool = true;
    const IS_NONE: bool = false;
    type None = [Some; 0]
    where
        (): StaticMaybe<Some>;
    type Some = [Some; 1];
    type Opposite = Self::None
    where
        (): StaticMaybe<Some>;
    type Maybe<M> = M
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
    type MaybeOr<M, O> = M
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> Some,
        Some: Sized
    {
        [func()]
    }

    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        core::mem::drop(or);
        maybe()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        let value = unsafe {
            core::ptr::read(&self[0])
        };
        core::mem::forget(self);
        StaticMaybe::maybe_from_fn(|| {
            map(value)
        })
    }
    fn maybe(self) -> Self::Maybe<Some>
    where
        Some: Sized + StaticMaybe<Some>,
        (): StaticMaybe<Some>,
        Self::Maybe<Some>: Sized
    {
        self.unwrap()
    }
}
impl<Some> /*const*/ StaticMaybe<Some> for MaybeCell<Some, true>
where
    Some: StaticMaybe<Some>
{
    const IS_SOME: bool = true;
    const IS_NONE: bool = false;
    type None = MaybeCell<Some, false>
    where
        (): StaticMaybe<Some>;
    type Some = MaybeCell<Some, true>;
    type Opposite = Self::None
    where
        (): StaticMaybe<Some>;
    type Maybe<M> = M
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
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
    
    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        core::mem::drop(or);
        maybe()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        StaticMaybe::maybe_from_fn(|| map(self.into_value()))
    }
    fn maybe(self) -> Self::Maybe<Some>
    where
        Some: Sized + StaticMaybe<Some>,
        (): StaticMaybe<Some>,
        Self::Maybe<Some>: Sized
    {
        self.into_value()
    }
}
impl<Some> /*const*/ StaticMaybe<Some> for MaybeCell<Some, false>
where
    Some: StaticMaybe<Some>
{
    const IS_SOME: bool = false;
    const IS_NONE: bool = true;
    type None = MaybeCell<Some, false>
    where
        (): StaticMaybe<Some>;
    type Some = MaybeCell<Some, true>;
    type Opposite = Self::Some
    where
        (): StaticMaybe<Some>;
    type Maybe<M> = ()
    where
        M: StaticMaybe<M> + ?Sized,
        (): StaticMaybe<M>;
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
    
    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>
    {
        core::mem::drop(maybe);
        or()
    }

    fn maybe_map<F>(self, map: F) -> Self::Maybe<F::Output>
    where
        F: FnOnce<(Some,)>,
        F::Output: StaticMaybe<F::Output>,
        (): StaticMaybe<F::Output>,
        Some: Sized,
        Self::Maybe<F::Output>: Sized
    {
        core::mem::drop(map);
        StaticMaybe::maybe_from_fn(|| panic!())
    }
    fn maybe(self) -> Self::Maybe<Some>
    where
        Some: Sized + StaticMaybe<Some>,
        (): StaticMaybe<Some>,
        Self::Maybe<Some>: Sized
    {
        
    }
}

impl<T> !StaticMaybe<T> for Option<T> {}