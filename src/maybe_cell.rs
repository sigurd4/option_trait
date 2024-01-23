use core::{mem::MaybeUninit, ops::{Deref, DerefMut}};

use crate::Maybe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaybeCell<T, const IS_SOME: bool>([T; IS_SOME as usize])
where
    [(); IS_SOME as usize]:;

impl<T> MaybeCell<T, false>
{
    pub const fn new() -> Self
    {
        Self([])
    }
}
impl<T> MaybeCell<T, true>
{
    pub const fn new(value: T) -> Self
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

    pub const fn into_option(self) -> Option<T>
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
}

impl<T> From<T> for MaybeCell<T, true>
{
    fn from(value: T) -> Self
    {
        Self::new(value)
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
        Self::new()
    }
}
impl<T> Default for MaybeCell<T, true>
where
    T: Default
{
    fn default() -> Self
    {
        Self::new(T::default())
    }
}

impl<T, const IS_SOME: bool> Maybe<T> for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    fn into_option(self) -> Option<T>
    {
        self.into_option()
    }

    fn as_option(&self) -> Option<&T>
    {
        self.get()
    }

    fn as_option_mut(&mut self) -> Option<&mut T>
    {
        self.get_mut()
    }
}