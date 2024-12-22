use core::marker::StructuralPartialEq;

use crate::{private, PureMaybe};

/// A trait, only implemented by [`Option`](core::option::Option) of any type.
/// 
/// This is a run-time managed [`Maybe`](crate::Maybe). Wether or not it contains a value can be decided at run-time.
//#[const_trait]
pub trait Optional: private::Optional + PureMaybe<Self::Some> + From<Option<Self::Some>> + Into<Option<Self::Some>> + StructuralPartialEq
{
    /// The internal type that may or may not exist inside the option.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Optional>::Some, i32);
    /// ```
    type Some;

    /// Creates a new [`Optional`](Optional) from a value.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let some = <Option<i32> as Optional>::some(777);
    /// 
    /// assert!(some.is_some());
    /// assert_eq!(some, Some(777));
    /// ```
    fn some(some: Self::Some) -> Self;
    /// Creates a new empty [`Optional`](Optional).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let some = <Option<i32> as Optional>::none();
    /// 
    /// assert!(some.is_none());
    /// assert_eq!(some, None);
    /// ```
    fn none() -> Self;

    /// Inserts `value` into the option, then returns a mutable reference to it.
    ///
    /// If the option already contains a value, the old value is dropped.
    ///
    /// See also [`Optional::get_or_insert`], which doesn't update the value if
    /// the option already contains [`Some`](core::option::Option::Some).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut option = None;
    /// 
    /// Optional::insert(&mut option, 32);
    /// 
    /// assert_eq!(option, Some(32));
    /// 
    /// Optional::insert(&mut option, 64);
    /// 
    /// assert_eq!(option, Some(64));
    /// ```
    fn insert(&mut self, value: Self::Some) -> &mut Self::Some;
    /// Inserts `value` into the option if it is [`None`](core::option::Option::None), then
    /// returns a mutable reference to the contained value.
    ///
    /// See also [`Optional::insert`], which updates the value even if
    /// the option already contains [`Some`](core::option::Option::Some).
    ///
    /// # Examples
    ///
    /// ```
    /// use option_trait::*;
    /// 
    /// let mut option = None;
    /// 
    /// Optional::get_or_insert(&mut option, 32);
    /// 
    /// assert_eq!(option, Some(32));
    /// 
    /// Optional::get_or_insert(&mut option, 64);
    /// 
    /// assert_eq!(option, Some(32));
    /// ```
    fn get_or_insert(&mut self, value: Self::Some) -> &mut Self::Some;
    /// Inserts the default value into the option if it is [`None`](core::option::Option::None), then
    /// returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use option_trait::*;
    /// 
    /// let mut option = None;
    /// 
    /// Optional::get_or_insert_default(&mut option);
    /// 
    /// assert_eq!(option, Some(0));
    /// ```
    fn get_or_insert_default(&mut self) -> &mut Self::Some
    where
        Self::Some: Default;
    /// Inserts a value computed from `insert` into the option if it is [`None`](core::option::Option::None),
    /// then returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use option_trait::*;
    /// 
    /// let mut option = None;
    /// 
    /// Optional::get_or_insert_with(&mut option, || 32);
    /// 
    /// assert_eq!(option, Some(32));
    /// 
    /// Optional::get_or_insert_with(&mut option, || panic!("Won't happen"));
    /// 
    /// assert_eq!(option, Some(32));
    /// ```
    fn get_or_insert_with<F>(&mut self, insert: F) -> &mut Self::Some
    where
        F: FnOnce() -> Self::Some;
    /// Takes the value out of the option, leaving a [`None`](core::option::Option::None) in its place.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut option = Some(69);
    /// 
    /// let taken = Optional::take(&mut option);
    /// 
    /// assert_eq!(taken, Some(69));
    /// assert_eq!(option, None);
    /// ```
    fn take(&mut self) -> Self;
    /// Takes the value out of the option, but only if the predicate evaluates to
    /// `true` on a mutable reference to the value.
    ///
    /// In other words, replaces `self` with `None` if the predicate returns `true`.
    /// This method operates similar to [`Optional::take`] but conditional.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut option = Some(69);
    /// 
    /// let taken = Optional::take_if(&mut option, |x| x == 69);
    /// 
    /// assert_eq!(taken, Some(69));
    /// assert_eq!(option, None);
    /// ```
    fn take_if<P>(&mut self, predicate: P) -> Self
    where
        P: FnOnce(&mut Self::Some) -> bool;
    /// Replaces the actual value in the option by the value given in parameter,
    /// returning the old value if present,
    /// leaving a [`Some`](core::option::Option::Some) in its place without deinitializing either one.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut option = Some(69);
    /// 
    /// let replaced = Optional::replace(&mut option, 420);
    /// 
    /// assert_eq!(replaced, Some(69));
    /// assert_eq!(option, Some(420));
    /// ```
    fn replace(&mut self, value: Self::Some) -> Self;
}
impl<T> /*const*/ Optional for Option<T>
{
    type Some = T;
    
    fn some(some: <Option<T> as Optional>::Some) -> Self
    {
        Some(some)
    }
    fn none() -> Self
    {
        None
    }

    fn insert(&mut self, value: T) -> &mut T
    {
        self.insert(value)
    }
    fn get_or_insert(&mut self, value: T) -> &mut T
    {
        self.get_or_insert(value)
    }
    fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default
    {
        self.get_or_insert_default()
    }
    fn get_or_insert_with<F>(&mut self, insert: F) -> &mut T
    where
        F: FnOnce() -> T
    {
        self.get_or_insert_with(insert)
    }
    fn take(&mut self) -> Self
    {
        self.take()
    }
    fn take_if<P>(&mut self, predicate: P) -> Self
    where
        P: FnOnce(&mut T) -> bool
    {
        self.take_if(predicate)
    }
    fn replace(&mut self, value: T) -> Self
    {
        self.replace(value)
    }
}

#[cfg(test)]
mod test
{
    use static_assertions::*;

    use crate::Optional;

    assert_type_eq_all!(<Option<i32> as Optional>::Some, i32);
}