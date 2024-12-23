use core::{hash::Hash, cmp::Ordering, fmt::Debug, marker::StructuralPartialEq, ops::{Deref, DerefMut}, pin::Pin};

use crate::{ops::{MaybeAnd, MaybeAndThen, MaybeFilter, MaybeOr, MaybeXor}, Copied, Maybe, NotVoid, PureStaticMaybe, StaticMaybe};

/// A struct containing a value of type `T`, if the constant expression `IS_SOME` evaluates to `true`.
/// 
/// This is similar to [`Option`](core::option::Option), except wether or not it contains a value is determined at
/// compile-time.
/// 
/// # Examples
/// 
/// ```rust
/// use option_trait::*;
/// 
/// let f = || ":^)";
/// 
/// let empty = OptCell::<&str, false>::from_fn(f);
/// let full = OptCell::<&str, true>::from_fn(f);
/// 
/// assert!(empty.is_none());
/// 
/// assert!(full.is_some());
/// assert_eq!(full.unwrap(), ":^)");
/// ```
pub struct OptCell<T, const IS_SOME: bool>(<T as private::_Spec<IS_SOME>>::Pure);

impl<T, const IS_SOME: bool> NotVoid for OptCell<T, IS_SOME> {}

impl<T> OptCell<T, false>
{
    /// Creates an empty [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let empty = OptCell::<i32, false>::none();
    /// 
    /// assert!(empty.is_none());
    /// ```
    pub const fn none() -> Self
    {
        Self::assume_none()
    }

    /// Crates an empty [`OptCell`](OptCell) for types like those contained in `like`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// assert!(empty.is_none());
    /// ```
    pub const fn none_like<const IS_SOME: bool>(like: &OptCell<T, IS_SOME>) -> Self
    {
        let _ = like;
        Self::none()
    }

    /// Crates an empty [`OptCell`](OptCell) for the same type as `like`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let empty = OptCell::none_for(&777);
    /// 
    /// assert!(empty.is_none());
    /// ```
    pub const fn none_for(like: &T) -> Self
    {
        let _ = like;
        Self::none()
    }
}
impl<T> OptCell<T, true>
{
    /// Creates a [`OptCell`](OptCell) that contains a value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// 
    /// assert_eq!(maybe.unwrap(), 777);
    /// ```
    pub const fn some(value: T) -> Self
    {
        Self::assume_some(value)
    }

    /// Unwraps the cell into its internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.into_value(), 777);
    /// ```
    pub const fn into_value(self) -> T
    {
        self.unwrap()
    }
    /// Unwraps the cell into its internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.as_value(), &777);
    /// ```
    pub const fn as_value(&self) -> &T
    {
        self.unwrap_ref()
    }
    /// Unwraps the cell into its internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.as_value_mut(), &mut 777);
    /// ```
    pub const fn as_value_mut(&mut self) -> &mut T
    {
        self.unwrap_mut()
    }
}
impl<T, const IS_SOME: bool> OptCell<T, IS_SOME>
{
    /// Crates an [`OptCell`](OptCell) that may or may not contain a value from a functor.
    /// 
    /// Wether or not the cell contains a value depends entirely on the constant expression `IS_SOME`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let f = || ":^)";
    /// 
    /// let empty = OptCell::<&str, false>::from_fn(f);
    /// let full = OptCell::<&str, true>::from_fn(f);
    /// 
    /// assert!(empty.is_none());
    /// 
    /// assert!(full.is_some());
    /// assert_eq!(full.unwrap(), ":^)");
    /// ```
    pub fn from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T
    {
        if !IS_SOME
        {
            return Self::assume_none()
        }
        Self::assume_some(func())
    }

    /// Converts the [`OptCell`](OptCell) into an [`Option`](core::option::Option).
    /// 
    /// The conversion can only go one way, due to the nature of the container types being compile-time managed and run-time managed respectively.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_ref(), &777);
    /// 
    /// let option = maybe.option();
    /// 
    /// assert!(option.is_some());
    /// assert_eq!(option.unwrap(), 777);
    /// ```
    pub const fn option(self) -> Option<T>
    {
        if IS_SOME
        {
            Some(self.unwrap())
        }
        else
        {
            core::mem::forget(self);
            None
        }
    }

    /// Retrieves the internal value in the form of an [`Option`](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_ref(), &777);
    /// 
    /// let option = maybe.get();
    /// 
    /// assert!(option.is_some());
    /// assert_eq!(option.unwrap(), &777);
    /// ```
    pub const fn get(&self) -> Option<&T>
    {
        if IS_SOME
        {
            Some(self.unwrap_ref())
        }
        else
        {
            None
        }
    }

    /// Mutably retrieves the internal value in the form of an [`Option`](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_ref(), &777);
    /// 
    /// let option = maybe.get_mut();
    /// 
    /// assert!(option.is_some());
    /// assert_eq!(option.unwrap(), &mut 777);
    /// ```
    pub const fn get_mut(&mut self) -> Option<&mut T>
    {
        if IS_SOME
        {
            Some(self.unwrap_mut())
        }
        else
        {
            None
        }
    }
    /// Retrieves the pinned internal value in the form of an [`Option`](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    ///     
    /// let maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.as_ref().unwrap_pin_ref(), core::pin::pin!(777));
    /// 
    /// let option = maybe.as_ref().get_pin();
    /// 
    /// assert!(option.is_some());
    /// assert_eq!(option.unwrap(), core::pin::pin!(777));
    /// ```
    pub fn get_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        if IS_SOME
        {
            Some(unsafe {
                self.map_unchecked(|this| this.unwrap_ref())
            })
        }
        else
        {
            None
        }
    }
    /// Mutably retrieves the pinned internal value in the form of an [`Option`](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    ///     
    /// let mut maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.as_mut().unwrap_pin_mut(), core::pin::pin!(777));
    /// 
    /// let option = maybe.as_mut().get_pin_mut();
    /// 
    /// assert!(option.is_some());
    /// assert_eq!(option.unwrap(), core::pin::pin!(777));
    /// ```
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        if IS_SOME
        {
            Some(unsafe {
                self.map_unchecked_mut(|this| this.unwrap_mut())
            })
        }
        else
        {
            None
        }
    }

    /// Returns true if the cell contains a value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let empty = OptCell::<i32, false>::none();
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(!empty.is_some());
    /// assert!(maybe.is_some());
    /// 
    /// assert_eq!(maybe.unwrap(), 777);
    /// ```
    pub const fn is_some(&self) -> bool
    {
        IS_SOME
    }
    /// Returns true if the cell does not contain a value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let empty = OptCell::<i32, false>::none();
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(empty.is_none());
    /// assert!(!maybe.is_none());
    /// 
    /// assert_eq!(maybe.unwrap(), 777);
    /// ```
    pub const fn is_none(&self) -> bool
    {
        !IS_SOME
    }
    /// Retrieves the internal value in the form of a [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_ref(), &777);
    /// 
    /// let maybe_ref = maybe.as_ref();
    /// 
    /// assert!(maybe_ref.is_some());
    /// assert_eq!(maybe_ref.unwrap(), &777);
    /// ```
    pub const fn as_ref<'a>(&'a self) -> <Self as Maybe<T>>::AsRef<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(self.unwrap_ref())
    }
    /// Mutably retrieves the internal value in the form of a [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_mut(), &mut 777);
    /// 
    /// let maybe_ref = maybe.as_mut();
    /// 
    /// assert!(maybe_ref.is_some());
    /// assert_eq!(maybe_ref.unwrap(), &mut 777);
    /// ```
    pub const fn as_mut<'a>(&'a mut self) -> <Self as Maybe<T>>::AsMut<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(self.unwrap_mut())
    }
    /// Retrieves the pinned internal value in the form of an [`Option`](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    ///     
    /// let maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.as_ref().unwrap_pin_ref(), core::pin::pin!(777));
    /// 
    /// let maybe_ref = maybe.as_ref().as_pin_ref();
    /// 
    /// assert!(maybe_ref.is_some());
    /// assert_eq!(maybe_ref.unwrap(), core::pin::pin!(777));
    /// ```
    pub fn as_pin_ref<'a>(self: Pin<&'a Self>) -> <Self as Maybe<T>>::AsPinRef<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(self.unwrap_pin_ref())
    }
    /// Retrieves the pinned internal value in the form of an [`Option`](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    ///     
    /// let mut maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.as_mut().unwrap_pin_mut(), core::pin::pin!(777));
    /// 
    /// let maybe_ref = maybe.as_mut().as_pin_mut();
    /// 
    /// assert!(maybe_ref.is_some());
    /// assert_eq!(maybe_ref.unwrap(), core::pin::pin!(777));
    /// ```
    pub fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> <Self as Maybe<T>>::AsPinMut<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(self.unwrap_pin_mut())
    }
    /// Retrieves the pinned internal value in the form of a slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    ///     
    /// let maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_ref(), &777);
    /// 
    /// let slice = maybe.as_slice();
    /// 
    /// assert_eq!(slice.len(), 1);
    /// assert_eq!(slice, &[777]);
    /// ```
    pub const fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        if !IS_SOME
        {
            return &[]
        }
        core::slice::from_ref(self.unwrap_ref())
    }
    /// Retrieves the pinned internal value in the form of a slice.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    ///     
    /// let mut maybe = OptCell::some(777);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_mut(), &mut 777);
    /// 
    /// let slice = maybe.as_mut_slice();
    /// 
    /// assert_eq!(slice.len(), 1);
    /// assert_eq!(slice, &mut [777]);
    /// ```
    pub const fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        if !IS_SOME
        {
            return &mut []
        }
        core::slice::from_mut(self.unwrap_mut())
    }
    /// Unwraps the [`OptCell`](OptCell) and prints a user defined message upon failure.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.expect("This should never happen..."), 777);
    /// ```
    pub const fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        if !IS_SOME
        {
            Self::on_unwrap_empty_msg(msg)
        }
        self.unwrap()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value, if it exists. If not, it will result in an error.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap(), 777);
    /// ```
    pub const fn unwrap(self) -> T
    where
        T: Sized
    {
        if !IS_SOME
        {
            Self::on_unwrap_empty()
        }
        let x = crate::assume_same(unsafe {
            core::ptr::read(&self.0)
        });
        core::mem::forget(self);
        x
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value by reference, if it exists. If not, it will result in an error.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_ref(), &777);
    /// ```
    pub const fn unwrap_ref(&self) -> &T
    {
        if !IS_SOME
        {
            Self::on_unwrap_empty()
        }
        crate::assume_same_ref(&self.0)
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value by mutable reference, if it exists. If not, it will result in an error.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_mut(), &mut 777);
    /// ```
    pub const fn unwrap_mut(&mut self) -> &mut T
    {
        if !IS_SOME
        {
            Self::on_unwrap_empty()
        }
        crate::assume_same_mut(&mut self.0)
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its pinned internal value by reference, if it exists. If not, it will result in an error.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!(OptCell::some(777));
    /// let maybe = maybe.as_ref();
    /// 
    /// assert_eq!(&*maybe.unwrap_pin_ref(), &777);
    /// ```
    pub fn unwrap_pin_ref(self: Pin<&Self>) -> Pin<&T>
    {
        if !IS_SOME
        {
            Self::on_unwrap_empty()
        }
        unsafe {
            self.map_unchecked(|this| this.unwrap_ref())
        }
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its pinned internal value by mutable reference, if it exists. If not, it will result in an error.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// assert_eq!(&mut *maybe.unwrap_pin_mut(), &mut 777);
    /// ```
    pub fn unwrap_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>
    {
        if !IS_SOME
        {
            Self::on_unwrap_empty()
        }
        unsafe {
            self.map_unchecked_mut(|this| this.unwrap_mut())
        }
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value, if it exists. Otherwise returns `default`
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_or(666), 777);
    /// 
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_or(666), 666);
    /// ```
    pub /*const*/ fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        if !IS_SOME
        {
            return default
        }
        self.unwrap()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value by reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_ref_or(&666), &777);
    /// 
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_ref_or(&666), &666);
    /// ```
    pub const fn unwrap_ref_or<'a>(&'a self, default: &'a T) -> &'a T
    where
        T: 'a
    {
        if !IS_SOME
        {
            return default
        }
        self.unwrap_ref()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value by mutable reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_mut_or(&mut 666), &mut 777);
    /// 
    /// let mut empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_mut_or(&mut 666), &mut 666);
    /// ```
    pub const fn unwrap_mut_or<'a>(&'a mut self, default: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        if !IS_SOME
        {
            return default
        }
        self.unwrap_mut()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its pinned internal value by reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// let default = core::pin::pin!(666);
    /// assert_eq!(maybe.as_ref().unwrap_pin_ref_or(default.as_ref()), core::pin::pin!(777));
    /// 
    /// let empty = core::pin::pin!(OptCell::none_like(&*maybe));
    /// 
    /// assert_eq!(empty.as_ref().unwrap_pin_ref_or(default.as_ref()), core::pin::pin!(666));
    /// ```
    pub fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return default
        }
        self.unwrap_pin_ref()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its pinned internal value by mutable reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// let mut default = core::pin::pin!(666);
    /// assert_eq!(maybe.as_mut().unwrap_pin_mut_or(default.as_mut()), core::pin::pin!(777));
    /// 
    /// let mut empty = core::pin::pin!(OptCell::none_like(&*maybe));
    /// 
    /// assert_eq!(empty.as_mut().unwrap_pin_mut_or(default.as_mut()), core::pin::pin!(666));
    /// ```
    pub fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, default: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return default
        }
        self.unwrap_pin_mut()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_or_else(|| 666), 777);
    /// 
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_or_else(|| 666), 666);
    /// ```
    pub fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        if !IS_SOME
        {
            return default()
        }
        self.unwrap()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value by reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_ref_or_else(|| &666), &777);
    /// 
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_ref_or_else(|| &666), &666);
    /// ```
    pub fn unwrap_ref_or_else<'a, F>(&'a self, default: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        if !IS_SOME
        {
            return default()
        }
        self.unwrap_ref()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value by mutable reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(777);
    /// 
    /// let default = &mut 666;
    /// assert_eq!(maybe.unwrap_mut_or_else(|| default), &mut 777);
    /// 
    /// let mut empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_mut_or_else(|| default), &mut 666);
    /// ```
    pub fn unwrap_mut_or_else<'a, F>(&'a mut self, default: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        if !IS_SOME
        {
            return default()
        }
        self.unwrap_mut()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its pinned internal value by reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// let default = core::pin::pin!(666);
    /// assert_eq!(maybe.as_ref().unwrap_pin_ref_or_else(|| default.as_ref()), core::pin::pin!(777));
    /// 
    /// let empty = core::pin::pin!(OptCell::none_like(&*maybe));
    /// 
    /// assert_eq!(empty.as_ref().unwrap_pin_ref_or_else(|| default.as_ref()), core::pin::pin!(666));
    /// ```
    pub fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, default: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        if !IS_SOME
        {
            return default()
        }
        self.unwrap_pin_ref()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its pinned internal value by mutable reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!(OptCell::some(777));
    /// 
    /// let mut default = core::pin::pin!(666);
    /// assert_eq!(maybe.as_mut().unwrap_pin_mut_or_else(|| default.as_mut()), core::pin::pin!(777));
    /// 
    /// let mut empty = core::pin::pin!(OptCell::none_like(&*maybe));
    /// 
    /// assert_eq!(empty.as_mut().unwrap_pin_mut_or_else(|| default.as_mut()), core::pin::pin!(666));
    /// ```
    pub fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, default: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
    {
        if !IS_SOME
        {
            return default()
        }
        self.unwrap_pin_mut()
    }
    /// Unwraps the [`OptCell`](OptCell) and returns its internal value, if it exists. Otherwise returns [`T::default()`](core::default::Default::default)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// assert_eq!(maybe.unwrap_or_default(), 777);
    /// 
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// assert_eq!(empty.unwrap_or_default(), 0);
    /// ```
    pub fn unwrap_or_default(self) -> T
    where
        T: Sized + Default
    {
        if !IS_SOME
        {
            return T::default()
        }
        self.unwrap()
    }
    /// Maps the internal value using a mapping function.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(666);
    /// 
    /// let mapped = maybe.map(|x| x + 111);
    /// 
    /// assert_eq!(mapped, OptCell::some(777));
    /// ```
    pub fn map<U, F>(self, map: F) -> OptCell<U, IS_SOME>
    where
        F: FnOnce(T) -> U
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(map(self.unwrap()))
    }
    /// Maps the internal value using a mapping function, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(666);
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// let mapped_maybe = maybe.map_or(999, |x| x + 111);
    /// let mapped_empty = empty.map_or(999, |x| x + 111);
    /// 
    /// assert_eq!(mapped_maybe, 777);
    /// assert_eq!(mapped_empty, 999);
    /// ```
    pub fn map_or<U, F>(self, default: U, map: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized
    {
        if !IS_SOME
        {
            return default
        }
        map(self.unwrap())
    }
    /// Maps the internal value using a mapping function, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(666);
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// let mapped_maybe = maybe.map_or_else(|| 999, |x| x + 111);
    /// let mapped_empty = empty.map_or_else(|| 999, |x| x + 111);
    /// 
    /// assert_eq!(mapped_maybe, 777);
    /// assert_eq!(mapped_empty, 999);
    /// ```
    pub fn map_or_else<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized
    {
        if !IS_SOME
        {
            return default()
        }
        map(self.unwrap())
    }
    /// Converts the [`OptCell`](OptCell) into a [`Result`](core::result::Result), with value `Ok` if the cell contains a value, otherwise `Err` containing `error`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some("All right");
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// let mapped_maybe = maybe.ok_or("Wrong");
    /// let mapped_empty = empty.ok_or("Wrong");
    /// 
    /// assert_eq!(mapped_maybe, Ok("All right"));
    /// assert_eq!(mapped_empty, Err("Wrong"));
    /// ```
    pub fn ok_or<E>(self, error: E) -> Result<T, E>
    where
        T: Sized
    {
        if !IS_SOME
        {
            return Err(error)
        }
        Ok(self.unwrap())
    }
    /// Converts the [`OptCell`](OptCell) into a [`Result`](core::result::Result), with value `Ok` if the cell contains a value,
    /// otherwise `Err` containing the reuslt of `error`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some("All right");
    /// let empty = OptCell::none_like(&maybe);
    /// 
    /// let mapped_maybe = maybe.ok_or_else(|| "Wrong");
    /// let mapped_empty = empty.ok_or_else(|| "Wrong");
    /// 
    /// assert_eq!(mapped_maybe, Ok("All right"));
    /// assert_eq!(mapped_empty, Err("Wrong"));
    /// ```
    pub fn ok_or_else<E, F>(self, error: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized
    {
        if !IS_SOME
        {
            return Err(error())
        }
        Ok(self.unwrap())
    }
    /// Retrieves the [`Deref`](core::ops::Deref) result of the  internal value in the form of a [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let text = String::from("Sample text");
    /// 
    /// let maybe = OptCell::some(text);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_ref(), &String::from("Sample text"));
    /// 
    /// let maybe_ref = maybe.as_deref();
    /// 
    /// assert!(maybe_ref.is_some());
    /// assert_eq!(maybe_ref.unwrap(), "Sample text");
    /// ```
    pub const fn as_deref<'a>(&'a self) -> <Self as Maybe<T>>::AsDeref<'a>
    where
        T: ~const Deref + 'a
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(self.unwrap_ref().deref())
    }
    /// Retrieves the [`DerefMut`](core::ops::DerefMut) result of the  internal value in the form of a [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let text = String::from("Sample text");
    /// 
    /// let mut maybe = OptCell::some(text);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap_mut(), &mut String::from("Sample text"));
    /// 
    /// let maybe_ref = maybe.as_deref_mut();
    /// 
    /// assert!(maybe_ref.is_some());
    /// assert_eq!(maybe_ref.unwrap(), "Sample text");
    /// ```
    pub const fn as_deref_mut<'a>(&'a mut self) -> <Self as Maybe<T>>::AsDerefMut<'a>
    where
        T: ~const DerefMut + 'a
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(self.unwrap_mut().deref_mut())
    }
    /// Returns the last of the two maybes, if both have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.and(b), ());
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.and(b), ());
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.and(b), ());
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.and(b), "Second");
    /// ```
    pub fn and<Rhs>(self, other: Rhs) -> <<Self as Maybe<T>>::Pure as MaybeAnd<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeAnd<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::and(self, other)
    }
    /// Maps the value into a different maybe if it exists using a flatmap function.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe1 = OptCell::some("abcdefg");
    /// let maybe2 = OptCell::some("abcdef");
    /// 
    /// let result1 = maybe1.and_then::<&str, _>(|value| if value.len() > 6 {None} else {Some(value)});
    /// let result2 = maybe2.and_then::<&str, _>(|value| if value.len() > 6 {None} else {Some(value)});
    /// 
    /// assert_eq!(result1, None);
    /// assert_eq!(result2, Some("abcdef"));
    /// ```
    #[doc(alias = "flatmap")]
    pub fn and_then<U, F>(self, and_then: F) -> <<Self as Maybe<T>>::Pure as MaybeAndThen<T, U, <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure>>::Output
    where
        F: FnOnce<(T,), Output: Maybe<U>>,
        <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure: Sized,
        (): StaticMaybe<T> + StaticMaybe<U>,
        <<Self as Maybe<T>>::Pure as MaybeAndThen<T, U, <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure>>::Output: Sized
    {
        Maybe::and_then(self, and_then)
    }
    /// Filters the internal value depending on a predicate.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe1 = OptCell::some("abcdefg");
    /// let maybe2 = OptCell::some("abcdef");
    /// 
    /// let result1 = maybe1.filter(|value| value.len() <= 6);
    /// let result2 = maybe2.filter(|value| value.len() <= 6);
    /// 
    /// assert_eq!(result1, None);
    /// assert_eq!(result2, Some("abcdef"));
    /// ```
    pub fn filter<F>(self, predicate: F) -> <<Self as Maybe<T>>::Pure as MaybeFilter<T>>::Output
    where
        F: Fn(&T) -> bool,
        (): StaticMaybe<T>,
        <Self as Maybe<T>>::Pure: MaybeFilter<T> + Sized
    {
        Maybe::filter(self, predicate)
    }
    /// Returns the first of the two maybes, if any of them have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.or(b), ());
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.or(b), "First");
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.or(b), "Second");
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.or(b), "First");
    /// ```
    pub fn or<Rhs>(self, other: Rhs) -> <<Self as Maybe<T>>::Pure as MaybeOr<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeOr<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::or(self, other)
    }
    /// Returns the first of the two maybes, if any of them have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.or_else(|| b), ());
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.or_else(|| b), "First");
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.or_else(|| b), "Second");
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.or_else(|| b), "First");
    /// ```
    pub fn or_else<F>(self, or_else: F) -> <<Self as Maybe<T>>::Pure as MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure>>::Output
    where
        F: FnOnce<(), Output: Maybe<T, Pure: Sized>>,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure>>::Output: Sized
    {
        Maybe::or_else(self, or_else)
    }
    /// Returns the first of the two maybes, if exactly one of them have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.xor(b), ());
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::<&'static str, false>::none();
    /// 
    /// assert_eq!(a.xor(b), "First");
    /// 
    /// let a = OptCell::<&'static str, false>::none();
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.xor(b), "Second");
    /// 
    /// let a = OptCell::some("First");
    /// let b = OptCell::some("Second");
    /// 
    /// assert_eq!(a.xor(b), ());
    /// ```
    pub fn xor<Rhs>(self, other: Rhs) -> <<Self as Maybe<T>>::Pure as MaybeXor<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeXor<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::xor(self, other)
    }
    /// Copies the internal value in the form of a [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let value = 777;
    /// 
    /// let maybe = OptCell::some(&value);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap(), &value);
    /// 
    /// let copy = maybe.copied();
    /// 
    /// assert!(copy.is_some());
    /// assert_eq!(copy.unwrap(), 777);
    /// ```
    pub const fn copied(&self) -> <Self as Maybe<T>>::Copied
    where
        T: Copied<Output: Copy>,
        (): StaticMaybe<<T as Copied>::Output>
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(crate::copy_ref(self.unwrap_ref()))
    }
    /// Clones the internal value in the form of a [`OptCell`](OptCell).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let value = vec![1, 2, 3];
    /// 
    /// let maybe = OptCell::some(&value);
    /// 
    /// assert!(maybe.is_some());
    /// assert_eq!(maybe.unwrap(), &value);
    /// 
    /// let copy = maybe.cloned();
    /// 
    /// assert!(copy.is_some());
    /// assert_eq!(copy.unwrap(), vec![1, 2, 3]);
    /// ```
    pub fn cloned(&self) -> <Self as Maybe<T>>::Copied
    where
        T: Copied<Output: Clone>,
        (): StaticMaybe<<T as Copied>::Output>
    {
        if !IS_SOME
        {
            return OptCell::assume_none()
        }
        OptCell::assume_some(crate::clone_ref(self.unwrap_ref()))
    }

    /// Iterates on the maybe by reference.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = OptCell::some(777);
    /// 
    /// for &value in maybe.iter()
    /// {
    ///     assert_eq!(value, 777);
    /// }
    /// ```
    pub fn iter(&self) -> core::option::Iter<T>
    {
        unsafe {
            crate::transmute_same_size::<
                core::option::IntoIter<&T>,
                core::option::Iter<T>
            >(self.get().into_iter())
        }
    }

    /// Iterates on the maybe by reference.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = OptCell::some(666);
    /// 
    /// for value in maybe.iter_mut()
    /// {
    ///     *value = 777
    /// }
    /// 
    /// assert_eq!(maybe, OptCell::some(777));
    /// ```
    pub fn iter_mut(&mut self) -> core::option::IterMut<T>
    {
        unsafe {
            crate::transmute_same_size::<
                core::option::IntoIter<&mut T>,
                core::option::IterMut<T>
            >(self.get_mut().into_iter())
        }
    }

    const fn on_unwrap_empty_msg(msg: &str) -> !
    {
        panic!("{}", msg)
    }
    const fn on_unwrap_empty() -> !
    {
        panic!("called `OptCell::unwrap()` on a `None` value")
    }
    const fn assume_none() -> Self
    {
        if IS_SOME
        {
            const fn ct() -> !
            {
                panic!("Tried to assume None on Some.")
            }
            fn rt<T>() -> !
            {
                panic!("Tried to assume None on Some<{}>.", core::any::type_name::<T>())
            }
            #[allow(unused_unsafe)]
            unsafe {
                core::intrinsics::const_eval_select((), ct, rt::<T>)
            }
        }
        Self(crate::assume_same(()))
    }
    const fn assume_some(value: T) -> Self
    {
        if !IS_SOME
        {
            const fn ct() -> !
            {
                panic!("Tried to assume Some on None.")
            }
            fn rt<T>() -> !
            {
                panic!("Tried to assume Some<{}> on None.", core::any::type_name::<T>())
            }
            #[allow(unused_unsafe)]
            unsafe {
                core::intrinsics::const_eval_select((), ct, rt::<T>)
            }
        }
        Self(crate::assume_same(value))
    }
}

impl<T, const IS_SOME: bool> Clone for OptCell<T, IS_SOME>
where
    <T as private::_Spec<IS_SOME>>::Pure: Clone
{
    fn clone(&self) -> Self
    {
        OptCell(self.0.clone())
    }
}
impl<T, const IS_SOME: bool> Copy for OptCell<T, IS_SOME>
where
    <T as private::_Spec<IS_SOME>>::Pure: Copy
{
    
}
impl<T, const IS_SOME: bool> Hash for OptCell<T, IS_SOME>
where
    <T as private::_Spec<IS_SOME>>::Pure: Hash
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H)
    {
        self.0.hash(state);
    }
}
impl<T> Default for OptCell<T, false>
{
    fn default() -> Self
    {
        Self::none()
    }
}
impl<T, const IS_SOME: bool> IntoIterator for OptCell<T, IS_SOME>
{
    type Item = T;
    type IntoIter = core::option::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.option().into_iter()
    }
}
impl<'a, T, const IS_SOME: bool> IntoIterator for &'a OptCell<T, IS_SOME>
{
    type Item = &'a T;
    type IntoIter = core::option::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.iter()
    }
}
impl<'a, T, const IS_SOME: bool> IntoIterator for &'a mut OptCell<T, IS_SOME>
{
    type Item = &'a mut T;
    type IntoIter = core::option::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.iter_mut()
    }
}
impl<T> From<()> for OptCell<T, false>
{
    fn from((): ()) -> Self
    {
        Self::none()
    }
}
impl<T> From<T> for OptCell<T, true>
{
    fn from(value: T) -> Self
    {
        Self::some(value)
    }
}
impl<T, const IS_SOME: bool> From<OptCell<T, IS_SOME>> for Option<T>
{
    fn from(value: OptCell<T, IS_SOME>) -> Self
    {
        value.option()
    }
}
impl<'a, T, const IS_SOME: bool> From<&'a OptCell<T, IS_SOME>> for Option<&'a T>
{
    fn from(value: &'a OptCell<T, IS_SOME>) -> Self
    {
        value.get()
    }
}
impl<'a, T, const IS_SOME: bool> From<&'a mut OptCell<T, IS_SOME>> for Option<&'a mut T>
{
    fn from(value: &'a mut OptCell<T, IS_SOME>) -> Self
    {
        value.get_mut()
    }
}
impl<'a, T, const IS_SOME: bool> From<&'a OptCell<T, IS_SOME>> for OptCell<&'a T, IS_SOME>
{
    fn from(value: &'a OptCell<T, IS_SOME>) -> OptCell<&'a T, IS_SOME>
    {
        value.as_ref()
    }
}
impl<'a, T, const IS_SOME: bool> From<&'a mut OptCell<T, IS_SOME>> for OptCell<&'a mut T, IS_SOME>
{
    fn from(value: &'a mut OptCell<T, IS_SOME>) -> OptCell<&'a mut T, IS_SOME>
    {
        value.as_mut()
    }
}
impl<T, const IS_SOME: bool> StructuralPartialEq for OptCell<T, IS_SOME>
{

}
impl<T, U, const A: bool, const B: bool> PartialEq<OptCell<U, B>> for OptCell<T, A>
where
    <T as private::_Spec<A>>::Pure: PartialEq<<U as private::_Spec<B>>::Pure>
{
    fn eq(&self, other: &OptCell<U, B>) -> bool
    {
        if A != B || !A
        {
            return !A
        }
        self.0.eq(&other.0)
    }
}
impl<T, const IS_SOME: bool> Eq for OptCell<T, IS_SOME>
where
    <T as private::_Spec<IS_SOME>>::Pure: Eq
{
    
}
impl<T, U, const A: bool, const B: bool> PartialOrd<OptCell<U, B>> for OptCell<T, A>
where
    <T as private::_Spec<A>>::Pure: PartialOrd<<U as private::_Spec<B>>::Pure>
{
    fn partial_cmp(&self, other: &OptCell<U, B>) -> Option<Ordering>
    {
        match (A, B)
        {
            (true, true) => self.0.partial_cmp(&other.0),
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => Some(Ordering::Equal)
        }
    }
}
impl<T, const IS_SOME: bool> Ord for OptCell<T, IS_SOME>
where
    <T as private::_Spec<IS_SOME>>::Pure: Ord
{
    fn cmp(&self, other: &OptCell<T, IS_SOME>) -> Ordering
    {
        if !IS_SOME
        {
            return Ordering::Equal
        }
        self.0.cmp(&other.0)
    }
}

impl<T> const Deref for OptCell<T, true>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        self.as_value()
    }
}
impl<T> const DerefMut for OptCell<T, true>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.as_value_mut()
    }
}

impl<T, const IS_SOME: bool> Debug for OptCell<T, IS_SOME>
where
    <T as private::_Spec<IS_SOME>>::Pure: Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        if IS_SOME
        {
            f.debug_tuple("Some")
                .field(&self.0)
                .finish()
        }
        else
        {
            f.debug_tuple("None")
                .finish()
        }
    }
}

impl<T, const IS_SOME: bool> Maybe<T> for OptCell<T, IS_SOME>
{
    const IS_MAYBE_SOME: bool = IS_SOME;
    const IS_MAYBE_NONE: bool = !IS_SOME;

    type Pure = <T as private::_Spec<IS_SOME>>::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>;
    type PureRef<'a> = <Self::AsRef<'a> as Maybe<&'a T>>::Pure
    where
        Self: 'a,
        T: 'a;
    type PureMut<'a> = <Self::AsMut<'a> as Maybe<&'a mut T>>::Pure
    where
        Self: 'a,
        T: 'a;
    type PurePinRef<'a> = <Self::AsPinRef<'a> as Maybe<Pin<&'a T>>>::Pure
    where
        Self: 'a,
        T: 'a;
    type PurePinMut<'a> = <Self::AsPinMut<'a> as Maybe<Pin<&'a mut T>>>::Pure
    where
        Self: 'a,
        T: 'a;

    type Mapped<U> = OptCell<U, IS_SOME>
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<<T as Copied>::Output>
    where
        T: Copied,
        (): StaticMaybe<<T as Copied>::Output>;

    fn is_some(&self) -> bool
    {
        self.is_some()
    }
    fn is_none(&self) -> bool
    {
        self.is_none()
    }
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a
    {
        self.as_ref()
    }
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a
    {
        self.as_mut()
    }
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a
    {
        self.as_pin_ref()
    }
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a
    {
        self.as_pin_mut()
    }
    fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        self.as_slice()
    }
    fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        self.as_mut_slice()
    }
    fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        self.expect(msg)
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        self.unwrap()
    }
    fn unwrap_ref(&self) -> &T
    {
        self.unwrap_ref()
    }
    fn unwrap_mut(&mut self) -> &mut T
    {
        self.unwrap_mut()
    }
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a
    {
        self.unwrap_pin_ref()
    }
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self.unwrap_pin_mut()
    }
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        self.unwrap_or(default)
    }
    fn unwrap_ref_or<'a>(&'a self, default: &'a T) -> &'a T
    where
        T: 'a
    {
        self.unwrap_ref_or(default)
    }
    fn unwrap_mut_or<'a>(&'a mut self, default: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        self.unwrap_mut_or(default)
    }
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        self.unwrap_pin_ref_or(default)
    }
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, default: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self.unwrap_pin_mut_or(default)
    }
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        self.unwrap_or_else(default)
    }
    fn unwrap_ref_or_else<'a, F>(&'a self, default: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        self.unwrap_ref_or_else(default)
    }
    fn unwrap_mut_or_else<'a, F>(&'a mut self, default: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        self.unwrap_mut_or_else(default)
    }
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, default: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        self.unwrap_pin_ref_or_else(default)
    }
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, default: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
    {
        self.unwrap_pin_mut_or_else(default)
    }
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default
    {
        self.unwrap_or_default()
    }
    fn map<U, F>(self, map: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>
    {
        self.map(map)
    }
    fn map_or<U, F>(self, default: U, map: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized
    {
        self.map_or(default, map)
    }
    fn map_or_else<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized
    {
        self.map_or_else(default, map)
    }
    fn ok_or<E>(self, error: E) -> Result<T, E>
    where
        T: Sized
    {
        self.ok_or(error)
    }
    fn ok_or_else<E, F>(self, error: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized
    {
        self.ok_or_else(error)
    }
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a
    {
        self.as_deref()
    }
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a
    {
        self.as_deref_mut()
    }
    fn copied(&self) -> Self::Copied
    where
        T: Copied<Output: Copy>,
        (): StaticMaybe<<T as Copied>::Output>
    {
        self.copied()
    }
    fn cloned(&self) -> Self::Copied
    where
        T: Copied<Output: Clone>,
        (): StaticMaybe<<T as Copied>::Output>
    {
        self.cloned()
    }
    
    fn option(self) -> Option<T>
    {
        self.option()
    }
    fn option_ref(&self) -> Option<&T>
    {
        self.get()
    }
    fn option_mut(&mut self) -> Option<&mut T>
    {
        self.get_mut()
    }
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        self.get_pin()
    }
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        self.get_pin_mut()
    }

    fn pure(self) -> Self::Pure
    where
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self::Pure: Sized
    {
        self.0
    }
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(&self.0)
    }
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(&mut self.0)
    }
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(unsafe {
            self.map_unchecked(|this| &this.0)
        })
    }
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(unsafe {
            self.map_unchecked_mut(|this| &mut this.0)
        })
    }
}

impl<T, const IS_SOME: bool> /*const*/ StaticMaybe<T> for OptCell<T, IS_SOME>
where
    T: StaticMaybe<T>
{
    const IS_SOME: bool = IS_SOME;
    const IS_NONE: bool = !IS_SOME;
    type None = OptCell<T, false>
    where
        (): StaticMaybe<T>;
    type Some = OptCell<T, true>;
    type Opposite = <T as private::_Spec<IS_SOME>>::Opposite
    where
        (): StaticMaybe<T>;
    type Maybe<M> = <T as private::_Spec<IS_SOME>>::Maybe<M>
    where
        M: ?Sized,
        (): PureStaticMaybe<M>;
    type MaybeOr<M, O> = <T as private::_Spec<IS_SOME>>::MaybeOr<M, O>
    where
        M: ?Sized,
        O: ?Sized;

    fn maybe_from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T,
        T: Sized
    {
        Self::from_fn(func)
    }
    
    fn maybe_or_from_fn<M, O>(maybe: M, or: O) -> Self::MaybeOr<M::Output, O::Output>
    where
        M: FnOnce<()>,
        O: FnOnce<()>,
        Self::MaybeOr<M::Output, O::Output>: Sized
    {
        if !IS_SOME
        {
            return crate::assume_same(or())
        }
        crate::assume_same(maybe())
    }

    fn into_value(self) -> T
    where
        Self: StaticMaybe<T, Maybe<T> = T>,
        T: Sized,
        (): PureStaticMaybe<T>,
        Self: Sized
    {
        self.unwrap()
    }
}

mod private
{
    use crate::{PureStaticMaybe, StaticMaybe};

    use super::OptCell;

    pub trait _Spec<const IS_SOME: bool>
    {
        type Opposite: StaticMaybe<Self>;
        type Maybe<M>: PureStaticMaybe<M> + ?Sized
        where
            M: ?Sized,
            (): PureStaticMaybe<M>;
        type MaybeOr<M, O>: ?Sized
        where
            M: ?Sized,
            O: ?Sized;
        type Pure: PureStaticMaybe<Self>;
    }
    impl<T, const IS_SOME: bool> _Spec<IS_SOME> for T
    {
        default type Opposite = OptCell<Self, false>;
        default type Maybe<M> = M
        where
            M: ?Sized,
            (): PureStaticMaybe<M>;
        default type MaybeOr<M, O> = M
        where
            M: ?Sized,
            O: ?Sized;
        default type Pure = T;
    }
    impl<T> _Spec<false> for T
    where
        (): PureStaticMaybe<T>
    {
        type Opposite = OptCell<Self, true>;
        type Maybe<M> = ()
        where
            M: ?Sized,
            (): PureStaticMaybe<M>;
        type MaybeOr<M, O> = O
        where
            M: ?Sized,
            O: ?Sized;
        type Pure = ();
    }
    impl<T> _Spec<true> for T
    where
        (): PureStaticMaybe<T>
    {
        type Opposite = OptCell<Self, false>;
        type Maybe<M> = M
        where
            M: ?Sized,
            (): PureStaticMaybe<M>;
        type MaybeOr<M, O> = M
        where
            M: ?Sized,
            O: ?Sized;
        type Pure = T;
    }
}

#[cfg(test)]
mod test
{
    use super::OptCell;

    #[test]
    fn it_works()
    {
        let maybe = OptCell::some(777);
        let empty = OptCell::none_like(&maybe);

        println!("{:?}", maybe);
        println!("{:?}", empty);
    }
}