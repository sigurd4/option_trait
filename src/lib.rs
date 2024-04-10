#![no_std]

#![feature(const_trait_impl)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_ptr_write)]

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

moddef::moddef!(
    flat(pub) mod {
        option_kind,
        option_obj,
        optional,
        maybe,
        maybe_cell,
        maybe_and,
        maybe_or,
        maybe_nand,
        maybe_nor,
        maybe_xor,
        maybe_xnor,
        static_maybe
    }
);

mod private
{
    use crate::MaybeCell;

    pub trait Optional {}
    impl<T> Optional for Option<T> {}

    pub auto trait NotVoid {}
    impl !NotVoid for () {}

    pub trait Maybe<T>
    where
        T: ?Sized {}
    impl<T> Maybe<T> for Option<T> {}
    impl<T> Maybe<T> for T
    where
        T: ?Sized {}
    impl<T> Maybe<T> for ()
    where
        T: NotVoid + ?Sized {}
    impl<T, const IS_SOME: bool> Maybe<T> for MaybeCell<T, IS_SOME>
    where
        [(); IS_SOME as usize]:
    {}
}