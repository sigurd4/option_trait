use core::{cmp::Ordering, fmt::Debug, marker::StructuralPartialEq, mem::MaybeUninit, ops::{Deref, DerefMut}, pin::Pin};

use crate::{ops::{MaybeAnd, MaybeAndThen, MaybeFilter, MaybeOr, MaybeXor}, Copied, Maybe, PureStaticMaybe, StaticMaybe};

#[derive(Copy, Hash)]
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
    pub const fn unwrap_ref(&self) -> &T
    {
        if !IS_SOME
        {
            panic!("called `MaybeCell::unwrap()` on a `None` value")
        }
        &self.0[0]
    }
    pub const fn unwrap_mut(&mut self) -> &mut T
    {
        if !IS_SOME
        {
            panic!("called `MaybeCell::unwrap()` on a `None` value")
        }
        &mut self.0[0]
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

    pub fn iter(&self) -> core::option::Iter<T>
    {
        unsafe {
            crate::transmute_same_size::<
                core::option::IntoIter<&T>,
                core::option::Iter<T>
            >(self.as_option().into_iter())
        }
    }

    pub fn iter_mut(&mut self) -> core::option::IterMut<T>
    {
        unsafe {
            crate::transmute_same_size::<
                core::option::IntoIter<&mut T>,
                core::option::IterMut<T>
            >(self.as_option_mut().into_iter())
        }
    }

    pub const fn voidify(self) -> MaybeCell<<T as private::_Spec<IS_SOME>>::Pure, IS_SOME>
    {
        unsafe {
            crate::transmute_same_size(self)
        }
    }
    pub const fn voidify_ref(&self) -> &MaybeCell<<T as private::_Spec<IS_SOME>>::Pure, IS_SOME>
    {
        unsafe {
            crate::transmute_same_size_ref(self)
        }
    }
    pub const fn voidify_mut(&mut self) -> &mut MaybeCell<<T as private::_Spec<IS_SOME>>::Pure, IS_SOME>
    {
        unsafe {
            crate::transmute_same_size_mut(self)
        }
    }
}

impl<T, const IS_SOME: bool> Clone for MaybeCell<T, IS_SOME>
where
    //<T as private::_Pure<IS_SOME>>::Pure: Clone,
    T: Clone,
    [(); IS_SOME as usize]:
{
    fn clone(&self) -> Self
    {
        let mut x = MaybeUninit::uninit_array();
        if IS_SOME
        {
            unsafe {
                x.as_mut_ptr()
                    .write(MaybeUninit::new(crate::assume_same(
                        self.unwrap_ref()
                            .clone()
                    )))
            };
        }
        MaybeCell(unsafe {
            MaybeUninit::array_assume_init(x)
        })
    }
}
impl<T> Default for MaybeCell<T, false>
{
    fn default() -> Self
    {
        Self::none()
    }
}
impl<T, const IS_SOME: bool> IntoIterator for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    type Item = T;
    type IntoIter = core::option::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.option().into_iter()
    }
}
impl<'a, T, const IS_SOME: bool> IntoIterator for &'a MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    type Item = &'a T;
    type IntoIter = core::option::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.iter()
    }
}
impl<'a, T, const IS_SOME: bool> IntoIterator for &'a mut MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    type Item = &'a mut T;
    type IntoIter = core::option::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.iter_mut()
    }
}
impl<T> From<T> for MaybeCell<T, true>
{
    fn from(value: T) -> Self
    {
        Self::some(value)
    }
}
impl<T, const IS_SOME: bool> Into<Option<T>> for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    fn into(self) -> Option<T>
    {
        self.option()
    }
}
impl<'a, T, const IS_SOME: bool> Into<Option<&'a T>> for &'a MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    fn into(self) -> Option<&'a T>
    {
        self.as_option()
    }
}
impl<'a, T, const IS_SOME: bool> Into<Option<&'a mut T>> for &'a mut MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    fn into(self) -> Option<&'a mut T>
    {
        self.as_option_mut()
    }
}
impl<'a, T, const IS_SOME: bool> From<&'a MaybeCell<T, IS_SOME>> for MaybeCell<&'a T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    fn from(value: &'a MaybeCell<T, IS_SOME>) -> MaybeCell<&'a T, IS_SOME>
    {
        value.as_ref()
    }
}
impl<'a, T, const IS_SOME: bool> From<&'a mut MaybeCell<T, IS_SOME>> for MaybeCell<&'a mut T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
    fn from(value: &'a mut MaybeCell<T, IS_SOME>) -> MaybeCell<&'a mut T, IS_SOME>
    {
        value.as_mut()
    }
}
impl<T, const IS_SOME: bool> StructuralPartialEq for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{

}
impl<T, U, const A: bool, const B: bool> PartialEq<MaybeCell<U, B>> for MaybeCell<T, A>
where
    [(); A as usize]:,
    [(); B as usize]:,
    <T as private::_Spec<A>>::Pure: PartialEq<<U as private::_Spec<B>>::Pure>
{
    fn eq(&self, other: &MaybeCell<U, B>) -> bool
    {
        if A != B || !A
        {
            return !A
        }
        self.voidify_ref()
            .unwrap_ref()
            .eq(other.voidify_ref()
                .unwrap_ref()
            )
    }

    fn ne(&self, other: &MaybeCell<U, B>) -> bool
    {
        if A != B || !A
        {
            return A
        }
        self.voidify_ref()
            .unwrap_ref()
            .ne(other.voidify_ref()
                .unwrap_ref()
            )
    }
}
impl<T, const IS_SOME: bool> Eq for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:,
    <T as private::_Spec<IS_SOME>>::Pure: Eq
{
    
}
impl<T, U, const A: bool, const B: bool> PartialOrd<MaybeCell<U, B>> for MaybeCell<T, A>
where
    [(); A as usize]:,
    [(); B as usize]:,
    <T as private::_Spec<A>>::Pure: PartialOrd<<U as private::_Spec<B>>::Pure>
{
    #[inline]
    fn partial_cmp(&self, other: &MaybeCell<U, B>) -> Option<Ordering>
    {
        match (A, B)
        {
            (true, true) => self.voidify_ref().unwrap_ref().partial_cmp(other.voidify_ref().unwrap_ref()),
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => Some(Ordering::Equal)
        }
    }
}
impl<T, const IS_SOME: bool> Ord for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:,
    <T as private::_Spec<IS_SOME>>::Pure: Ord
{
    #[inline]
    fn cmp(&self, other: &MaybeCell<T, IS_SOME>) -> Ordering
    {
        if !IS_SOME
        {
            return Ordering::Equal
        }
        self.voidify_ref().unwrap_ref().cmp(other.voidify_ref().unwrap_ref())
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

impl<T, const IS_SOME: bool> Debug for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:,
    <T as private::_Spec<IS_SOME>>::Pure: Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {
        if IS_SOME
        {
            f.debug_tuple("Some")
                .field(self.voidify_ref().unwrap_ref())
                .finish()
        }
        else
        {
            f.debug_tuple("None")
                .finish()
        }
    }
}

impl<T, const IS_SOME: bool> Maybe<T> for MaybeCell<T, IS_SOME>
where
    [(); IS_SOME as usize]:
{
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

mod private
{
    use crate::{NotVoid, PureStaticMaybe, StaticMaybe};

    use super::MaybeCell;

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
        default type Opposite = MaybeCell<Self, false>;
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
        T: NotVoid
    {
        type Opposite = MaybeCell<Self, true>;
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
}

impl<T, const IS_SOME: bool> /*const*/ StaticMaybe<T> for MaybeCell<T, IS_SOME>
where
    T: StaticMaybe<T>,
    [(); IS_SOME as usize]:
{
    const IS_SOME: bool = IS_SOME;
    const IS_NONE: bool = !IS_SOME;
    type None = MaybeCell<T, false>
    where
        (): StaticMaybe<T>;
    type Some = MaybeCell<T, true>;
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