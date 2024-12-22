use core::{cell::UnsafeCell, marker::PhantomData, mem::ManuallyDrop, ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive, Yeet}, pin::Pin};

/// A stupid trait made to avoid conflicting implementations.
/// 
/// This is supposed to be implemented for anything that isn't `()`, but sometimes it is not, if you make a new struct that contains `()` in any kind of way.
/// 
/// If you do, it is highly encouraged to implement this trait for your struct manually, so it can be used with the [`Maybe`](crate::Maybe) trait.
/// 
/// Currently this is the best solution i could find to avoid a conflicting implementation, but it may be subject to change later.
/// Ideally i'd want to get rid of this thing.
pub auto trait NotVoid {}
impl !NotVoid for () {}

impl<T, const N: usize> NotVoid for [T; N] {}
impl<T> NotVoid for [T] {}

impl<T> NotVoid for PhantomData<T> {}
impl<T> NotVoid for UnsafeCell<T> {}
impl<T> NotVoid for ManuallyDrop<T> {}
impl<T> NotVoid for Range<T> {}
impl<T> NotVoid for RangeFrom<T> {}
impl<T> NotVoid for RangeInclusive<T> {}
impl<T> NotVoid for RangeTo<T> {}
impl<T> NotVoid for RangeToInclusive<T> {}
impl<T> NotVoid for Yeet<T> {}
impl<T> NotVoid for Pin<T> {}
impl<T> NotVoid for Bound<T> {}
impl<T> NotVoid for Option<T> {}
impl<T, E> NotVoid for Result<T, E> {}

impl<T> NotVoid for &T where T: ?Sized {}
impl<T> NotVoid for &mut T where T: ?Sized {}
impl<T> NotVoid for *const T where T: ?Sized {}
impl<T> NotVoid for *mut T where T: ?Sized {}

// Failed attemts to implement this in a "more" better way.

/*pub trait NotVoid: private::_MaybeVoid<IS_VOID = false>
{

}
impl<T> NotVoid for T
where
    T: private::_MaybeVoid<IS_VOID = false> + ?Sized
{

}

mod private
{
    pub trait _MaybeVoid
    {
        const IS_VOID: bool;
    }
    impl<T> _MaybeVoid for T
    where
        T: ?Sized
    {
        default const IS_VOID: bool = false;
    }
    impl _MaybeVoid for ()
    {
        const IS_VOID: bool = true;
    }
}*/

/*pub trait NotVoid: private::_NotVoid<Void = ()>
{

}
impl<T> NotVoid for T
where
    T: private::_NotVoid<Void = ()>
{
    
}
impl !NotVoid for ()
{
    
}

mod private
{
    pub trait _NotVoid
    {
        type Void;
    }
    impl<T> _NotVoid for T
    {
        default type Void = ();
    }
    impl _NotVoid for ()
    {
        type Void = !;
    }
}*/