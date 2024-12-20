#![no_std]
#![allow(internal_features)]
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
#![feature(specialization)]
#![feature(generic_const_exprs)]

moddef::moddef!(
    pub mod {
        ops
    },
    flat(pub) mod {
        option_kind,
        option_obj,
        optional,
        maybe,
        maybe_cell,
        pure_maybe,
        pure_static_maybe,
        not_void,
        static_maybe
    }
);

const unsafe fn transmute_same_size<T, U>(value: T) -> U
{
    assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    unsafe { core::intrinsics::transmute_unchecked::<T, U>(value) }
}
const unsafe fn transmute_same_size_ref<T, U>(value: &T) -> &U
{
    assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    unsafe { core::mem::transmute::<&T, &U>(value) }
}
const unsafe fn transmute_same_size_mut<T, U>(value: &mut T) -> &mut U
{
    assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    unsafe { core::mem::transmute::<&mut T, &mut U>(value) }
}

const fn is_same_type<T, U>() -> bool
where
    T: ?Sized,
    U: ?Sized
{
    assert!(<T as private::MaybeSame::<U>>::IS_SAME == <U as private::MaybeSame::<T>>::IS_SAME);
    <T as private::MaybeSame<U>>::IS_SAME
}

const fn assume_same<T, U>(value: T) -> U
{
    assert!(is_same_type::<T, U>());
    unsafe { core::intrinsics::transmute_unchecked::<T, U>(value) }
}
const fn assume_same_ref<T, U>(value: &T) -> &U
{
    assert!(is_same_type::<T, U>());
    unsafe { core::intrinsics::transmute::<&T, &U>(value) }
}
const fn copy_ref<T>(src: &T) -> Copied<T>
where
    Copied<T>: Copy
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

pub trait Same<T>: private::Same<T> {}
impl<T, U> Same<T> for U where U: private::Same<T> {}

pub type Copied<T> = <T as private::Copied>::Copied;

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

    pub trait Copied
    {
        type Copied;
    }
    impl<T> Copied for T
    {
        default type Copied = T;
    }
    impl<T> Copied for &T
    {
        type Copied = T;
    }
    impl<T> Copied for &mut T
    {
        type Copied = T;
    }

    use crate::{MaybeCell, NotVoid};

    pub trait Optional {}
    impl<T> Optional for Option<T> {}

    pub trait Maybe<T>
    where
        T: ?Sized
    {
    }
    impl<T> Maybe<T> for Option<T> {}
    impl<T> Maybe<T> for T where T: ?Sized {}
    impl<T> Maybe<T> for () where T: NotVoid + ?Sized {}
    impl<T> Maybe<T> for [T; 0] {}
    impl<T> Maybe<T> for [T; 1] {}
    impl<T, const IS_SOME: bool> Maybe<T> for MaybeCell<T, IS_SOME> where [(); IS_SOME as usize]: {}

    pub trait PureMaybe<T>: Maybe<T>
    where
        T: ?Sized
    {
    }
    impl<T> PureMaybe<T> for Option<T> {}
    impl<T> PureMaybe<T> for T where T: ?Sized {}
    impl<T> PureMaybe<T> for () where T: NotVoid + ?Sized {}
}
