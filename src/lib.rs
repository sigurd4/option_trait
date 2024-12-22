#![cfg_attr(not(test), no_std)]
#![allow(internal_features)]
#![allow(clippy::type_complexity)]
#![feature(const_trait_impl)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(try_trait_v2_yeet)]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(const_destruct)]
#![feature(adt_const_params)]
#![feature(associated_const_equality)]
#![feature(structural_match)]
#![feature(core_intrinsics)]
#![feature(const_eval_select)]
#![feature(never_type)]
#![feature(specialization)]
#![feature(generic_const_exprs)]

//! Provides the [`Optional`](Optional) trait for [`Option`](core::option::Option)s, as well as compile-time managed [`Option`](core::option::Option) alternatives,
//! all generalized under the trait [`Maybe`](Maybe).
//!
//! [`Maybe<T>`](Maybe) is implemented for:
//! - [`Option<T>`](core::option::Option)
//!     - Run-time managed
//!     - Also implements [`Optional`](Optional) and [`PureMaybe`](PureMaybe)
//! - `T` and `()`
//!     - Compile-time managed
//!     - Also implements [`PureStaticMaybe`](PureStaticMaybe), [`PureMaybe`](PureMaybe) and [`StaticMaybe`](StaticMaybe)
//! - `[T; 1]` and `[T; 0]`
//!     - Compile-time managed
//!     - Can be managed using constant expressions, but with some difficulty
//!     - Also implements [`StaticMaybe`](StaticMaybe)
//! - [`OptCell<T, _>`](OptCell) (`feature = "opt_cell"`)
//!     - Compile-time managed
//!     - Can be more easily managed using boolean constant expressions
//!     - Has const methods
//!     - Also implements [`StaticMaybe`](StaticMaybe)
//!
//! # Examples
//!
//! This is how i like to handle optional function arguments with maximum flexibility.
//!
//! ```rust
//! use option_trait::*;
//!
//! fn f<O>(required: i32, optional: O)
//! where
//!     O: Maybe<i32>
//! {
//!     if O::IS_MAYBE_SOME
//!     {
//!         // This part of the code will be disabled at compile-time if the maybe cannot possibly contain a value.
//!     }
//!
//!     // Do whatever
//! }
//!
//! f(1, 2);
//! f(1, ());
//! f(1, Some(2));
//! f(1, None);
//! f(1, [2]);
//! f(1, [] as [i32; 0]);
//! f(1, OptCell::some(2));
//! f(1, OptCell::none());
//! ```

moddef::moddef!(
    pub mod {
        ops
    },
    flat(pub) mod {
        optional,
        maybe,
        pure_maybe,
        static_maybe,
        pure_static_maybe,
        opt_cell for cfg(feature = "opt_cell"),
        not_void
    }
);

#[allow(unused)]
use crate as option_trait;

#[allow(unused)]
const unsafe fn transmute_same_size<T, U>(value: T) -> U
{
    assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    unsafe { core::intrinsics::transmute_unchecked::<T, U>(value) }
}
/*#[allow(unused)]
const unsafe fn transmute_same_size_ref<T, U>(value: &T) -> &U
{
    assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    unsafe { core::mem::transmute::<&T, &U>(value) }
}*/
/*#[allow(unused)]
const unsafe fn transmute_same_size_mut<T, U>(value: &mut T) -> &mut U
{
    assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    unsafe { core::mem::transmute::<&mut T, &mut U>(value) }
}*/

const fn is_same_type<T, U>() -> bool
where
    T: ?Sized,
    U: ?Sized
{
    assert!(<T as private::MaybeSame::<U>>::IS_SAME == <U as private::MaybeSame::<T>>::IS_SAME);
    <T as private::MaybeSame<U>>::IS_SAME
}

#[allow(unused)]
const fn assume_same<T, U>(value: T) -> U
{
    assert!(is_same_type::<T, U>());
    unsafe { core::intrinsics::transmute_unchecked::<T, U>(value) }
}
#[allow(unused)]
const fn assume_same_ref<T, U>(value: &T) -> &U
{
    assert!(is_same_type::<T, U>());
    unsafe { core::intrinsics::transmute::<&T, &U>(value) }
}
#[allow(unused)]
const fn assume_same_mut<T, U>(value: &mut T) -> &mut U
{
    assert!(is_same_type::<T, U>());
    unsafe { core::intrinsics::transmute::<&mut T, &mut U>(value) }
}
const fn copy_ref<T>(src: &T) -> Copied<T>
where
    <T as private::_Copied>::Copied: Copy
{
    if is_same_type::<T, Copied<T>>()
    {
        return *assume_same_ref::<T, Copied<T>>(src);
    }
    assert!(is_same_type::<T, &Copied<T>>() || is_same_type::<T, &mut Copied<T>>());
    unsafe { **core::intrinsics::transmute::<&T, &&Copied<T>>(src) }
}
fn clone_ref<T>(src: &T) -> Copied<T>
where
    Copied<T>: Clone
{
    if is_same_type::<T, Copied<T>>()
    {
        return assume_same_ref::<T, Copied<T>>(src).clone();
    }
    assert!(is_same_type::<T, &Copied<T>>() || is_same_type::<T, &mut Copied<T>>());
    unsafe { (*core::intrinsics::transmute::<&T, &&Copied<T>>(src)).clone() }
}
const fn on_unwrap_empty() -> !
{
    panic!("called `Maybe::unwrap()` on a `None` value")
}
const fn on_unwrap_empty_msg(msg: &str) -> !
{
    panic!("{}", msg)
}

pub trait Same<T>: private::Same<T> {}
impl<T, U> Same<T> for U where U: private::Same<T> {}

pub type Copied<T> = <T as private::_Copied>::Copied;

mod private
{
    pub trait MaybeSame<T>
    where
        T: ?Sized
    {
        const IS_SAME: bool;
    }
    impl<T, U> MaybeSame<U> for T
    where
        T: ?Sized,
        U: ?Sized
    {
        default const IS_SAME: bool = false;
    }
    impl<T> MaybeSame<T> for T
    where
        T: ?Sized
    {
        const IS_SAME: bool = true;
    }

    pub trait Same<T> {}
    impl<T, U> Same<T> for U where T: MaybeSame<T, IS_SAME = true> {}

    pub trait _Ref
    {
        type Target;
    }
    impl<T> _Ref for &T
    {
        type Target = T;
    }
    impl<T> _Ref for &mut T
    {
        type Target = T;
    }

    pub trait _Copied
    {
        type Copied;
    }
    impl<T> _Copied for T
    {
        default type Copied = T;
    }
    impl<T> _Copied for T
    where
        T: _Ref
    {
        type Copied = <T as _Ref>::Target;
    }

    use crate::NotVoid;

    pub trait Optional {}
    impl<T> Optional for Option<T> {}

    pub trait PureMaybe<T>
    where
        T: ?Sized
    {
    }
    impl<T> PureMaybe<T> for Option<T> {}
    impl<T> PureMaybe<T> for T where T: ?Sized {}
    impl<T> PureMaybe<T> for () where T: NotVoid + ?Sized {}
}

#[cfg(test)]
mod test
{
    use static_assertions::assert_type_eq_all;

    use crate::option_trait;
    use option_trait::*;

    assert_type_eq_all!(<&i32 as private::_Copied>::Copied, i32);
    //assert_type_eq_all!(<i32 as private::_Copied>::Copied, i32);

    #[test]
    fn it_works()
    {
        use option_trait::*;
        use static_assertions::*;

        assert_type_eq_all!(<Option<&i32> as Maybe<&i32>>::Copied, Option<i32>);
        assert_type_eq_all!(<&i32 as Maybe<&i32>>::Copied, i32);
        assert_type_eq_all!(<() as Maybe<&i32>>::Copied, ());
        assert_type_eq_all!(<[&i32; 1] as Maybe<&i32>>::Copied, [i32; 1]);
        assert_type_eq_all!(<[&i32; 0] as Maybe<&i32>>::Copied, [i32; 0]);

        // This is supposed to work...
        /*assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Copied, Option<i32>);
        assert_type_eq_all!(<i32 as Maybe<i32>>::Copied, i32);
        assert_type_eq_all!(<() as Maybe<i32>>::Copied, ());
        assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Copied, [i32; 1]);
        assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Copied, [i32; 0]);*/

        let maybe = [777];
        let referenced = Maybe::<i32>::as_ref(&maybe);

        // This is supposed to work...
        //let copy1 = Maybe::<i32>::copied(&maybe);
        //assert_eq!(copy1, [777]);

        let copy2 = Maybe::<&i32>::copied(&referenced);
        assert_eq!(copy2, [777]);
    }

    #[test]
    fn pinned()
    {
        use option_trait::*;

        let maybe = core::pin::pin!(777);

        assert!(maybe.is_some());
        assert_eq!(**maybe.unwrap_ref(), 777);

        let option = maybe.option_pin_mut();

        assert!(option.is_some());
        assert_eq!(*option.unwrap(), 777);
    }
}
