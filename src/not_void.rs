use core::{cell::UnsafeCell, marker::PhantomData, mem::ManuallyDrop, ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive, Yeet}, pin::Pin};

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