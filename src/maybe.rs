use core::{ops::{Deref, DerefMut}, pin::Pin};

use crate::{ops::{MaybeAnd, MaybeAndThen, MaybeFilter, MaybeOr, MaybeXor}, private, Copied, NotVoid, PureMaybe, StaticMaybe};

pub trait Maybe<T>
where
    T: ?Sized
{
    type Pure: PureMaybe<T> + ?Sized
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>;
    type PureRef<'a>: PureMaybe<&'a T>
    where
        Self: 'a,
        T: 'a;
    type PureMut<'a>: PureMaybe<&'a mut T>
    where
        Self: 'a,
        T: 'a;
    type PurePinRef<'a>: PureMaybe<Pin<&'a T>>
    where
        Self: 'a,
        T: 'a;
    type PurePinMut<'a>: PureMaybe<Pin<&'a mut T>>
    where
        Self: 'a,
        T: 'a;

    type Mapped<U>: Maybe<U>
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied: Maybe<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;
    type AsRef<'a>: Maybe<&'a T> + 'a = Self::Mapped<&'a T>
    where
        Self: 'a,
        T: 'a;
    type AsMut<'a>: Maybe<&'a mut T> + 'a = Self::Mapped<&'a mut T>
    where
        Self: 'a,
        T: 'a;
    type AsPinRef<'a>: Maybe<Pin<&'a T>> + 'a = Self::Mapped<Pin<&'a T>>
    where
        Self: 'a,
        T: 'a;
    type AsPinMut<'a>: Maybe<Pin<&'a mut T>> + 'a = Self::Mapped<Pin<&'a mut T>>
    where
        Self: 'a,
        T: 'a;
    type AsDeref<'a>: Maybe<&'a <T as Deref>::Target> + 'a = Self::Mapped<&'a <T as Deref>::Target>
    where
        Self: 'a,
        T: Deref + 'a;
    type AsDerefMut<'a>: Maybe<&'a mut <T as Deref>::Target> + 'a = Self::Mapped<&'a mut <T as Deref>::Target>
    where
        Self: 'a,
        T: Deref + 'a;

    fn is_some(&self) -> bool;
    fn is_none(&self) -> bool;
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a;
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a;
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a;
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a;
    fn maybe_as_slice(&self) -> &[T]
    where
        T: Sized;
    fn maybe_as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized;
    fn expect(self, msg: &str) -> T
    where
        T: Sized;
    fn unwrap(self) -> T
    where
        T: Sized;
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized;
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized;
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default;
    fn map<U, F>(self, map: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    fn map_or<U, F>(self, default: U, map: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized;
    fn map_or_else<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized;
    fn ok_or<E>(self, error: E) -> Result<T, E>
    where
        T: Sized;
    fn ok_or_else<E, F>(self, error: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized;
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a;
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a;
    fn and<Rhs>(self, other: Rhs) -> <Self::Pure as MaybeAnd<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Self: Sized,
        Rhs::Pure: Sized,
        Self::Pure: MaybeAnd<T, Rhs::Pure> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        <Self::Pure as MaybeAnd<T, Rhs::Pure>>::Output: Sized
    {
        MaybeAnd::and(self.pure_maybe(), other.pure_maybe())
    }
    fn and_then<U, F>(self, and_then: F) -> <Self::Pure as MaybeAndThen<T, U, <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure>>::Output
    where
        F: FnOnce<(T,), Output: Maybe<U>>,
        Self: Sized,
        <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure: Sized,
        Self::Pure: MaybeAndThen<T, U, <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure> + Sized,
        T: StaticMaybe<T> + Sized,
        U: StaticMaybe<U> + Sized,
        (): StaticMaybe<T> + StaticMaybe<U>,
        <Self::Pure as MaybeAndThen<T, U, <<F as FnOnce<(T,)>>::Output as Maybe<U>>::Pure>>::Output: Sized
    {
        MaybeAndThen::and_then(self.pure_maybe(), |x| and_then(x).pure_maybe())
    }
    fn filter<F>(self, predicate: F) -> <Self::Pure as MaybeFilter<T>>::Output
    where
        Self: Sized,
        F: Fn(&T) -> bool,
        Self::Pure: MaybeFilter<T> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
    {
        MaybeFilter::filter(self.pure_maybe(), predicate)
    }
    fn or<Rhs>(self, other: Rhs) -> <Self::Pure as MaybeOr<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Self: Sized,
        Rhs::Pure: Sized,
        Self::Pure: MaybeOr<T, Rhs::Pure> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        <Self::Pure as MaybeOr<T, Rhs::Pure>>::Output: Sized
    {
        MaybeOr::or(self.pure_maybe(), other.pure_maybe())
    }
    fn or_else<F>(self, or_else: F) -> <Self::Pure as MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure>>::Output
    where
        F: FnOnce<(), Output: Maybe<T, Pure: Sized>>,
        Self: Sized,
        Self::Pure: MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        <Self::Pure as MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure>>::Output: Sized
    {
        MaybeOr::or_else(self.pure_maybe(), || or_else().pure_maybe())
    }
    fn xor<Rhs>(self, other: Rhs) -> <Self::Pure as MaybeXor<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Self: Sized,
        Rhs::Pure: Sized,
        Self::Pure: MaybeXor<T, Rhs::Pure> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        <Self::Pure as MaybeXor<T, Rhs::Pure>>::Output: Sized
    {
        MaybeXor::xor(self.pure_maybe(), other.pure_maybe())
    }
    fn copied(&self) -> Self::Copied
    where
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>;
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>;

    fn option(self) -> Option<T>
    where
        T: Sized;
    fn as_option(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>;
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>;

    fn pure_maybe(self) -> Self::Pure
    where
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self::Pure: Sized;
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a;
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a;
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a;
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a;
}
impl<T> /*const*/ Maybe<T> for T
where
    T: ?Sized
{
    type Pure = T
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

    type Mapped<U> = U
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;

    fn is_some(&self) -> bool
    {
        true
    }
    fn is_none(&self) -> bool
    {
        false
    }
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a
    {
        self
    }
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a
    {
        self
    }
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked(|this| this.as_ref())
        }
    }
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked_mut(|this| this.as_mut())
        }
    }
    fn maybe_as_slice(&self) -> &[T]
    where
        T: Sized
    {
        core::slice::from_ref(self)
    }
    fn maybe_as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        core::slice::from_mut(self)
    }
    fn expect(self, _: &str) -> T
    where
        T: Sized
    {
        self
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        self
    }
    fn unwrap_or(self, _: T) -> T
    where
        T: Sized
    {
        self
    }
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        core::mem::drop(default);
        self
    }
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default
    {
        self
    }
    fn map<U, F>(self, map: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>
    {
        map(self)
    }
    fn map_or<U, F>(self, _: U, map: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized
    {
        map(self)
    }
    fn map_or_else<U, D, F>(self, _: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized
    {
        map(self)
    }
    fn ok_or<E>(self, _: E) -> Result<T, E>
    where
        T: Sized
    {
        Ok(self)
    }
    fn ok_or_else<E, F>(self, _: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized
    {
        Ok(self)
    }
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a
    {
        self.deref()
    }
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a
    {
        self.deref_mut()
    }
    fn copied(&self) -> Self::Copied
    where
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        crate::copy_ref(self)
    }
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        crate::clone_ref(self)
    }

    fn option(self) -> Option<T>
    where
        T: Sized
    {
        Some(self)
    }
    fn as_option(&self) -> Option<&T>
    {
        Some(self)
    }
    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        Some(self)
    }
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        Some(self)
    }
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        Some(self)
    }

    fn pure_maybe(self) -> Self::Pure
    where
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self::Pure: Sized
    {
        crate::assume_same(self)
    }
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        self
    }
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        self
    }
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        self
    }
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {
        self
    }
}
impl<T> /*const*/ Maybe<T> for ()
where
    T: NotVoid + ?Sized
{
    type Pure = ();
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

    type Mapped<U> = ()
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;

    fn is_some(&self) -> bool
    {
        false
    }
    fn is_none(&self) -> bool
    {
        true
    }
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a
    {
        
    }
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a
    {
        
    }
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a
    {
        
    }
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a
    {
        
    }
    fn maybe_as_slice(&self) -> &[T]
    where
        T: Sized
    {
        &[]
    }
    fn maybe_as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        &mut []
    }
    fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        [].expect(msg)
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        [].unwrap()
    }
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        default
    }
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        default()
    }
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default
    {
        T::default()
    }
    fn map<U, F>(self, _: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>
    {
        
    }
    fn map_or<U, F>(self, default: U, _: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized
    {
        default
    }
    fn map_or_else<U, D, F>(self, default: D, _: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized
    {
        default()
    }
    fn ok_or<E>(self, error: E) -> Result<T, E>
    where
        T: Sized
    {
        Err(error)
    }
    fn ok_or_else<E, F>(self, error: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized
    {
        Err(error())
    }
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a
    {
        
    }
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a
    {
        
    }
    fn copied(&self) -> Self::Copied
    where
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        
    }
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        
    }

    fn option(self) -> Option<T>
    where
        T: Sized
    {
        None
    }
    fn as_option(&self) -> Option<&T>
    {
        None
    }
    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        None
    }
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        None
    }
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        None
    }

    fn pure_maybe(self) -> Self::Pure
    {

    }
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {

    }
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {

    }
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {

    }
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {

    }
}
impl<T> /*const*/ Maybe<T> for Option<T>
{
    type Pure = Option<T>
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

    type Mapped<U> = Option<U>
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;

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
    fn maybe_as_slice(&self) -> &[T]
    where
        T: Sized
    {
        self.as_slice()
    }
    fn maybe_as_mut_slice(&mut self) -> &mut [T]
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
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        self.unwrap_or(default)
    }
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        self.unwrap_or_else(default)
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
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        self.as_ref()
            .map(crate::copy_ref)
    }
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        self.as_ref()
            .map(crate::clone_ref)
    }

    fn option(self) -> Option<T>
    {
        self
    }
    fn as_option(&self) -> Option<&T>
    {
        self.as_ref()
    }
    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        self.as_mut()
    }
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        self.as_pin_ref()
    }
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        self.as_pin_mut()
    }

    fn pure_maybe(self) -> Self::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>
    {
        self
    }
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        self.as_ref()
    }
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        self.as_mut()
    }
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        self.as_pin_ref()
    }
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {
        self.as_pin_mut()
    }
}
impl<T> /*const*/ Maybe<T> for [T; 0]
{
    type Pure = <() as Maybe<T>>::Pure
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

    type Mapped<U> = [U; 0]
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;

    fn is_some(&self) -> bool
    {
        false
    }
    fn is_none(&self) -> bool
    {
        true
    }
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a
    {
        []
    }
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a
    {
        []
    }
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a
    {
        []
    }
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a
    {
        []
    }
    fn maybe_as_slice(&self) -> &[T]
    where
        T: Sized
    {
        &[]
    }
    fn maybe_as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        &mut []
    }
    fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        panic!("{}", msg)
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        panic!("called `Maybe::unwrap()` on a `None` value")
    }
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        default
    }
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        default()
    }
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default
    {
        T::default()
    }
    fn map<U, F>(self, _: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>
    {
        []
    }
    fn map_or<U, F>(self, default: U, _: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized
    {
        default
    }
    fn map_or_else<U, D, F>(self, default: D, _: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized
    {
        default()
    }
    fn ok_or<E>(self, error: E) -> Result<T, E>
    where
        T: Sized
    {
        Err(error)
    }
    fn ok_or_else<E, F>(self, error: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized
    {
        Err(error())
    }
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a
    {
        []
    }
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a
    {
        []
    }
    fn copied(&self) -> Self::Copied
    where
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        []
    }
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        []
    }

    fn option(self) -> Option<T>
    where
        T: Sized
    {
        None
    }
    fn as_option(&self) -> Option<&T>
    {
        None
    }
    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        None
    }
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        None
    }
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        None
    }

    fn pure_maybe(self) -> Self::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>,
        Self::Pure: Sized
    {
        crate::assume_same(())
    }
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {

    }
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {

    }
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {

    }
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {

    }
}
impl<T> /*const*/ Maybe<T> for [T; 1]
{
    type Pure = T
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

    type Mapped<U> = [U; 1]
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;

    fn is_some(&self) -> bool
    {
        true
    }
    fn is_none(&self) -> bool
    {
        false
    }
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a
    {
        [&self[0]]
    }
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a
    {
        [&mut self[0]]
    }
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a
    {
        [
            unsafe {
                self.map_unchecked(|this| &this[0])
            }
        ]
    }
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a
    {
        [
            unsafe {
                self.map_unchecked_mut(|this| &mut this[0])
            }
        ]
    }
    fn maybe_as_slice(&self) -> &[T]
    where
        T: Sized
    {
        self
    }
    fn maybe_as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        self
    }
    fn expect(self, _: &str) -> T
    where
        T: Sized
    {
        self.unwrap()
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        let value = unsafe {
            core::ptr::read(&self[0])
        };
        core::mem::drop(self);
        value
    }
    fn unwrap_or(self, _: T) -> T
    where
        T: Sized
    {
        self.unwrap()
    }
    fn unwrap_or_else<F>(self, _: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        self.unwrap()
    }
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default
    {
        self.unwrap()
    }
    fn map<U, F>(self, map: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>
    {
        [map(self.unwrap())]
    }
    fn map_or<U, F>(self, _: U, map: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized
    {
        map(self.unwrap())
    }
    fn map_or_else<U, D, F>(self, _: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized
    {
        map(self.unwrap())
    }
    fn ok_or<E>(self, _: E) -> Result<T, E>
    where
        T: Sized
    {
        Ok(self.unwrap())
    }
    fn ok_or_else<E, F>(self, _: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized
    {
        Ok(self.unwrap())
    }
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a
    {
        [self[0].deref()]
    }
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a
    {
        [self[0].deref_mut()]
    }

    fn option(self) -> Option<T>
    where
        T: Sized
    {
        let value = unsafe {
            core::ptr::read(&self[0])
        };
        core::mem::forget(self);
        Some(value)
    }
    fn as_option(&self) -> Option<&T>
    {
        Some(&self[0])
    }
    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        Some(&mut self[0])
    }
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        Some(unsafe {
            self.map_unchecked(|this| &this[0])
        })
    }
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        Some(unsafe {
            self.map_unchecked_mut(|this| &mut this[0])
        })
    }
    fn copied(&self) -> Self::Copied
    where
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        [crate::copy_ref(&self[0])]
    }
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>
    {
        [crate::clone_ref(&self[0])]
    }

    fn pure_maybe(self) -> Self::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>
    {
        self.unwrap()
    }
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        &self[0]
    }
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        &mut self[0]
    }
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked(|this| &this[0])
        }
    }
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked_mut(|this| &mut this[0])
        }
    }
}