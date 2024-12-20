use core::{mem::MaybeUninit, ops::{Deref, DerefMut}, pin::Pin};

use crate::{ops::{MaybeAnd, MaybeAndThen, MaybeFilter, MaybeOr, MaybeXor}, Copied, Maybe, StaticMaybe};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaybeCell<T, const IS_SOME: bool>([T; IS_SOME as usize])
where
    [(); IS_SOME as usize]:;

impl<T> MaybeCell<T, false>
{
    pub const fn none() -> Self
    {
        Self([])
    }
}
impl<T> MaybeCell<T, true>
{
    pub const fn some(value: T) -> Self
    {
        Self([value])
    }

    pub const fn into_value(self) -> T
    {
        let x = unsafe {core::ptr::read(self.0.as_ptr())};
        core::mem::forget(self);
        x
    }
    pub const fn as_value(&self) -> &T
    {
        &self.0[0]
    }
    pub const fn as_value_mut(&mut self) -> &mut T
    {
        &mut self.0[0]
    }
}
impl<T, const IS_SOME: bool> MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    pub fn from_fn<F>(func: F) -> Self
    where
        F: FnOnce() -> T
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {x.as_mut_ptr().write(MaybeUninit::new(func()))};
        }
        Self(unsafe {MaybeUninit::array_assume_init(x)})
    }

    pub const fn option(self) -> Option<T>
    {
        if IS_SOME
        {
            let x = unsafe {core::ptr::read(self.0.as_ptr())};
            core::mem::forget(self);
            Some(x)
        }
        else
        {
            core::mem::forget(self);
            None
        }
    }

    pub const fn get(&self) -> Option<&T>
    {
        if IS_SOME
        {
            Some(&self.0[0])
        }
        else
        {
            None
        }
    }

    pub const fn get_mut(&mut self) -> Option<&mut T>
    {
        if IS_SOME
        {
            Some(&mut self.0[0])
        }
        else
        {
            None
        }
    }
    pub fn get_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        if IS_SOME
        {
            Some(unsafe {
                self.map_unchecked(|this| &this.0[0])
            })
        }
        else
        {
            None
        }
    }
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        if IS_SOME
        {
            Some(unsafe {
                self.map_unchecked_mut(|this| &mut this.0[0])
            })
        }
        else
        {
            None
        }
    }

    pub const fn is_some(&self) -> bool
    {
        IS_SOME
    }
    pub const fn is_none(&self) -> bool
    {
        !IS_SOME
    }
    pub const fn as_ref<'a>(&'a self) -> <Self as Maybe<T>>::AsRef<'a>
    where
        T: 'a
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(&self.0[0]))
            };
        }
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
    pub const fn as_mut<'a>(&'a mut self) -> <Self as Maybe<T>>::AsMut<'a>
    where
        T: 'a
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(&mut self.0[0]))
            };
        }
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
    pub fn as_pin_ref<'a>(self: Pin<&'a Self>) -> <Self as Maybe<T>>::AsPinRef<'a>
    where
        T: 'a
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(self.map_unchecked(|this| &this.0[0])))
            };
        }
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
    pub fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> <Self as Maybe<T>>::AsPinMut<'a>
    where
        T: 'a
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(self.map_unchecked_mut(|this| &mut this.0[0])))
            };
        }
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
    pub const fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        &self.0
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        &mut self.0
    }
    pub const fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        if !IS_SOME
        {
            panic!("{}", msg)
        }
        self.unwrap()
    }
    pub const fn unwrap(self) -> T
    where
        T: Sized
    {
        if !IS_SOME
        {
            panic!("called `MaybeCell::unwrap()` on a `None` value")
        }
        let x = unsafe {
            core::ptr::read(&self.0[0])
        };
        core::mem::forget(self);
        x
    }
    pub fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        if !IS_SOME
        {
            return default
        }
        self.unwrap()
    }
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
    pub fn map<U, F>(self, map: F) -> MaybeCell<U, IS_SOME>
    where
        F: FnOnce(T) -> U
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {x.as_mut_ptr().write(MaybeUninit::new(map(self.0.as_ptr().read())))};
        }
        core::mem::forget(self);
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
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
    pub const fn as_deref<'a>(&'a self) -> <Self as Maybe<T>>::AsDeref<'a>
    where
        T: ~const Deref + 'a
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(self.0[0].deref()))
            };
        }
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
    pub const fn as_deref_mut<'a>(&'a mut self) -> <Self as Maybe<T>>::AsDerefMut<'a>
    where
        T: ~const DerefMut + 'a
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(core::mem::transmute::<&mut T, &mut T>(&mut self.0[0]).deref_mut()))
            };
        }
        MaybeCell(unsafe {MaybeUninit::array_assume_init(x)})
    }
    pub fn and<Rhs>(self, other: Rhs) -> <<Self as Maybe<T>>::Pure as MaybeAnd<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeAnd<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::and(self, other)
    }
    pub fn and_then<F, U, Rhs>(self, and_then: F) -> <<Self as Maybe<T>>::Pure as MaybeAndThen<T, U, Rhs::Pure>>::Output
    where
        F: FnOnce(T) -> Rhs,
        Rhs: Maybe<U>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T> + StaticMaybe<U>,
        <<Self as Maybe<T>>::Pure as MaybeAndThen<T, U, Rhs::Pure>>::Output: Sized
    {
        Maybe::and_then(self, and_then)
    }
    pub fn filter<F>(self, predicate: F) -> <<Self as Maybe<T>>::Pure as MaybeFilter<T>>::Output
    where
        F: Fn(&T) -> bool,
        (): StaticMaybe<T>,
        <Self as Maybe<T>>::Pure: MaybeFilter<T> + Sized
    {
        Maybe::filter(self, predicate)
    }
    pub fn or<Rhs>(self, other: Rhs) -> <<Self as Maybe<T>>::Pure as MaybeOr<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeOr<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::or(self, other)
    }
    pub fn or_else<Rhs, F>(self, or_else: F) -> <<Self as Maybe<T>>::Pure as MaybeOr<T, Rhs::Pure>>::Output
    where
        F: FnOnce() -> Rhs,
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeOr<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::or_else(self, or_else)
    }
    pub fn xor<Rhs>(self, other: Rhs) -> <<Self as Maybe<T>>::Pure as MaybeXor<T, Rhs::Pure>>::Output
    where
        Rhs: Maybe<T>,
        Rhs::Pure: Sized,
        (): StaticMaybe<T>,
        <<Self as Maybe<T>>::Pure as MaybeXor<T, Rhs::Pure>>::Output: Sized
    {
        Maybe::xor(self, other)
    }
    pub const fn copied(&self) -> <Self as Maybe<T>>::Copied
    where
        Copied<T>: Copy,
        (): StaticMaybe<Copied<T>>
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(crate::copy_ref(&self.0[0])))
            };
        }
        MaybeCell(unsafe {
            MaybeUninit::array_assume_init(x)
        })
    }
    pub fn cloned(&self) -> <Self as Maybe<T>>::Copied
    where
        Copied<T>: Clone,
        (): StaticMaybe<Copied<T>>
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr().write(MaybeUninit::new(crate::clone_ref(&self.0[0])))
            };
        }
        MaybeCell(unsafe {
            MaybeUninit::array_assume_init(x)
        })
    }
}

impl<T> From<T> for MaybeCell<T, true>
{
    fn from(value: T) -> Self
    {
        Self::some(value)
    }
}
impl<T> Deref for MaybeCell<T, true>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        self.as_value()
    }
}
impl<T> DerefMut for MaybeCell<T, true>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.as_value_mut()
    }
}

impl<T> Default for MaybeCell<T, false>
{
    fn default() -> Self
    {
        Self::none()
    }
}
impl<T> Default for MaybeCell<T, true>
where
    T: Default
{
    fn default() -> Self
    {
        Self::some(T::default())
    }
}

mod private
{
    use crate::{PureMaybe, NotVoid};

    pub trait Pure<const IS_SOME: bool>
    {
        type Pure: PureMaybe<Self>;
    }
    impl<T, const IS_SOME: bool> Pure<IS_SOME> for T
    {
        default type Pure = T;
    }
    impl<T> Pure<false> for T
    where
        T: NotVoid
    {
        type Pure = ();
    }
}

impl<T, const IS_SOME: bool> Maybe<T> for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    type Pure = <T as private::Pure<IS_SOME>>::Pure
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

    type Mapped<U> = MaybeCell<U, IS_SOME>
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    type Copied = Self::Mapped<Copied<T>>
    where
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
        crate::Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<crate::Copied<T>>
    {
        self.copied()
    }
    fn cloned(&self) -> Self::Copied
    where
        crate::Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<crate::Copied<T>>
    {
        self.cloned()
    }
    
    fn option(self) -> Option<T>
    {
        self.option()
    }
    fn as_option(&self) -> Option<&T>
    {
        self.get()
    }
    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        self.get_mut()
    }
    fn as_option_pin(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        self.get_pin()
    }
    fn as_option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        self.get_pin_mut()
    }

    fn pure_maybe(self) -> Self::Pure
    where
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self::Pure: Sized
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(self.unwrap())
    }
    fn as_pure_maybe<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(&self.0[0])
    }
    fn as_pure_maybe_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(&mut self.0[0])
    }
    fn as_pure_maybe_pin<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(unsafe {
            self.map_unchecked(|this| &this.0[0])
        })
    }
    fn as_pure_maybe_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {
        if !IS_SOME
        {
            return crate::assume_same(())
        }
        crate::assume_same(unsafe {
            self.map_unchecked_mut(|this| &mut this.0[0])
        })
    }
}