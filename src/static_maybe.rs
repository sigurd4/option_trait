use crate::{Maybe, NotVoid, PureStaticMaybe};

/// A trait for [`Maybe`](crate::Maybe)-types that are compile-time managed.
pub trait StaticMaybe<T>: Maybe<T>
where
    T: ?Sized
{
    /// Equals `true` if the [`Maybe`](crate::Maybe)-type contains a value.
    const IS_SOME: bool;
    /// Equals `true` if the [`Maybe`](crate::Maybe)-type does not contain a value.
    const IS_NONE: bool;
    /// This kind of maybe if it contained a value.
    type Some: StaticMaybe<T> + ?Sized;
    /// This kind of maybe if it didn't contain a value.
    type None: StaticMaybe<T>
    where
        (): StaticMaybe<T>;
    /// This kind of maybe which is full if this one is empty, or empty if this one is full.
    type Opposite: StaticMaybe<T> + ?Sized
    where
        (): StaticMaybe<T>;
    /// `M` if this contains a value, otherwise `()`.
    type Maybe<M>: PureStaticMaybe<M> + ?Sized
    where
        M: ?Sized,
        (): PureStaticMaybe<M>;
    /// `M` if this contains a value, otherwise `O`.
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
        O: FnOnce<()>,
        Self::MaybeOr<M::Output, O::Output>: Sized;

    /// Unwraps the maybe into its inner value. This one won't panic, as opposed to [`Maybe::unwrap()`](crate::Maybe::unwrap).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = ["turnip"];
    /// 
    /// let value = StaticMaybe::<&str>::into_value(maybe);
    /// 
    /// assert_eq!(value, "turnip");
    /// ```
    fn into_value(self) -> T
    where
        Self: StaticMaybe<T, Maybe<T> = T>,
        (): PureStaticMaybe<T>,
        T: Sized,
        Self: Sized
    {
        self.unwrap()
    }
    /// Unwraps the maybe into its inner value by reference. This one won't panic, as opposed to [`Maybe::unwrap_ref()`](crate::Maybe::unwrap_ref).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = ["turnip"];
    /// 
    /// let value = StaticMaybe::<&str>::as_value_ref(&maybe);
    /// 
    /// assert_eq!(value, &"turnip");
    /// ```
    fn as_value(&self) -> &T
    where
        Self: StaticMaybe<T, Maybe<T> = T>,
        (): PureStaticMaybe<T>
    {
        self.unwrap_ref()
    }
    /// Unwraps the maybe into its inner value by mutable reference. This one won't panic, as opposed to [`Maybe::unwrap_mut()`](crate::Maybe::unwrap_mut).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = ["turnip"];
    /// 
    /// let value = StaticMaybe::<&str>::as_value_mut(&mut maybe);
    /// 
    /// assert_eq!(value, &"turnip");
    /// ```
    fn as_value_mut(&mut self) -> &mut T
    where
        Self: StaticMaybe<T, Maybe<T> = T>,
        (): PureStaticMaybe<T>
    {
        self.unwrap_mut()
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
        M: ?Sized,
        (): PureStaticMaybe<M>;
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
        M: ?Sized,
        (): PureStaticMaybe<M>;
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
        M: ?Sized,
        (): PureStaticMaybe<M>;
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
        M: ?Sized,
        (): PureStaticMaybe<M>;
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
}

impl<T> !StaticMaybe<T> for Option<T> {}