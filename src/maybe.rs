use core::{ops::{Deref, DerefMut}, pin::Pin};

use crate::{ops::{MaybeAnd, MaybeAndThen, MaybeFilter, MaybeOr, MaybeXor}, Copied, NotVoid, PureMaybe, StaticMaybe};

#[cfg(test)]
mod test
{
    use core::pin::Pin;

    use crate::*;
    use static_assertions::*;

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Pure, Option<i32>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::Pure, i32);
    assert_type_eq_all!(<() as Maybe<i32>>::Pure, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Pure, i32);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Pure, ());

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PureRef<'static>, Option<&i32>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::PureRef<'static>, &i32);
    assert_type_eq_all!(<() as Maybe<i32>>::PureRef<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PureRef<'static>, &i32);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PureRef<'static>, ());

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PureMut<'static>, Option<&mut i32>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::PureMut<'static>, &mut i32);
    assert_type_eq_all!(<() as Maybe<i32>>::PureMut<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PureMut<'static>, &mut i32);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PureMut<'static>, ());

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PurePinRef<'static>, Option<Pin<&i32>>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::PurePinRef<'static>, Pin<&i32>);
    assert_type_eq_all!(<() as Maybe<i32>>::PurePinRef<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PurePinRef<'static>, Pin<&i32>);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PurePinRef<'static>, ());

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PurePinMut<'static>, Option<Pin<&mut i32>>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::PurePinMut<'static>, Pin<&mut i32>);
    assert_type_eq_all!(<() as Maybe<i32>>::PurePinMut<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PurePinMut<'static>, Pin<&mut i32>);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PurePinMut<'static>, ());

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Mapped<u64>, Option<u64>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::Mapped<u64>, u64);
    assert_type_eq_all!(<() as Maybe<i32>>::Mapped<u64>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Mapped<u64>, [u64; 1]);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Mapped<u64>, [u64; 0]);

    assert_type_eq_all!(<Option<&i32> as Maybe<&i32>>::Copied, Option<i32>);
    assert_type_eq_all!(<&i32 as Maybe<&i32>>::Copied, i32);
    assert_type_eq_all!(<() as Maybe<&i32>>::Copied, ());
    assert_type_eq_all!(<[&i32; 1] as Maybe<&i32>>::Copied, [i32; 1]);
    assert_type_eq_all!(<[&i32; 0] as Maybe<&i32>>::Copied, [i32; 0]);
    
    // This is supposed to work, but the compiler gets confused...
    // Wait for specialization to be a stable feature.
    /*assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Copied, Option<i32>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::Copied, i32);
    assert_type_eq_all!(<() as Maybe<i32>>::Copied, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Copied, [i32; 1]);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Copied, [i32; 0]);*/
    
    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsRef<'static>, Option<&i32>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::AsRef<'static>, &i32);
    assert_type_eq_all!(<() as Maybe<i32>>::AsRef<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsRef<'static>, [&i32; 1]);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsRef<'static>, [&i32; 0]);

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsMut<'static>, Option<&mut i32>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::AsMut<'static>, &mut i32);
    assert_type_eq_all!(<() as Maybe<i32>>::AsMut<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsMut<'static>, [&mut i32; 1]);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsMut<'static>, [&mut i32; 0]);

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsPinRef<'static>, Option<Pin<&i32>>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::AsPinRef<'static>, Pin<&i32>);
    assert_type_eq_all!(<() as Maybe<i32>>::AsPinRef<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsPinRef<'static>, [Pin<&i32>; 1]);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsPinRef<'static>, [Pin<&i32>; 0]);

    assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsPinMut<'static>, Option<Pin<&mut i32>>);
    assert_type_eq_all!(<i32 as Maybe<i32>>::AsPinMut<'static>, Pin<&mut i32>);
    assert_type_eq_all!(<() as Maybe<i32>>::AsPinMut<'static>, ());
    assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsPinMut<'static>, [Pin<&mut i32>; 1]);
    assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsPinMut<'static>, [Pin<&mut i32>; 0]);

    assert_type_eq_all!(<Option<Box<i32>> as Maybe<Box<i32>>>::AsDeref<'static>, Option<&i32>);
    assert_type_eq_all!(<Box<i32> as Maybe<Box<i32>>>::AsDeref<'static>, &i32);
    assert_type_eq_all!(<() as Maybe<Box<i32>>>::AsDeref<'static>, ());
    assert_type_eq_all!(<[Box<i32>; 1] as Maybe<Box<i32>>>::AsDeref<'static>, [&i32; 1]);
    assert_type_eq_all!(<[Box<i32>; 0] as Maybe<Box<i32>>>::AsDeref<'static>, [&i32; 0]);
}

pub trait Maybe<T>
where
    T: ?Sized
{
    /// Either `T`, `()` or an [Option](core::option::Option) containing `T`.
    /// 
    /// This is an option if the maybe is run-time managed, otherwise it's the inner type or void if compile-time managed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Pure, Option<i32>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::Pure, i32);
    /// assert_type_eq_all!(<() as Maybe<i32>>::Pure, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Pure, i32);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Pure, ());
    /// 
    /// let maybe = [777];
    /// 
    /// let pure = Maybe::<i32>::pure(maybe);
    /// 
    /// assert_eq!(pure, 777);
    /// ```
    type Pure: PureMaybe<T> + ?Sized
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>;
    /// Either `&T`, `()` or an [Option](core::option::Option) containing `&T`.
    /// 
    /// This is an option if the maybe is run-time managed, otherwise it's the inner type or void if compile-time managed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PureRef<'static>, Option<&i32>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::PureRef<'static>, &i32);
    /// assert_type_eq_all!(<() as Maybe<i32>>::PureRef<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PureRef<'static>, &i32);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PureRef<'static>, ());
    /// 
    /// let maybe = [777];
    /// 
    /// let pure = Maybe::<i32>::pure_ref(&maybe);
    /// 
    /// assert_eq!(pure, &777);
    /// ```
    type PureRef<'a>: PureMaybe<&'a T>
    where
        Self: 'a,
        T: 'a;
    /// Either `&mut T`, `()` or an [Option](core::option::Option) containing `&mut T`.
    /// 
    /// This is an option if the maybe is run-time managed, otherwise it's the inner type or void if compile-time managed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PureMut<'static>, Option<&mut i32>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::PureMut<'static>, &mut i32);
    /// assert_type_eq_all!(<() as Maybe<i32>>::PureMut<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PureMut<'static>, &mut i32);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PureMut<'static>, ());
    /// 
    /// let mut maybe = [777];
    /// 
    /// let pure = Maybe::<i32>::pure_mut(&mut maybe);
    /// 
    /// assert_eq!(pure, &mut 777);
    /// ```
    type PureMut<'a>: PureMaybe<&'a mut T>
    where
        Self: 'a,
        T: 'a;
    /// Either [Pin<&T>](core::pin::Pin), `()` or an [Option](core::option::Option) containing [Pin<&T>](core::pin::Pin).
    /// 
    /// This is an option if the maybe is run-time managed, otherwise it's the inner type or void if compile-time managed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// use core::pin::Pin;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PurePinRef<'static>, Option<Pin<&i32>>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::PurePinRef<'static>, Pin<&i32>);
    /// assert_type_eq_all!(<() as Maybe<i32>>::PurePinRef<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PurePinRef<'static>, Pin<&i32>);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PurePinRef<'static>, ());
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// 
    /// let pure = Maybe::<i32>::pure_pin_ref(maybe.as_ref());
    /// 
    /// assert_eq!(&*pure, &777);
    /// ```
    type PurePinRef<'a>: PureMaybe<Pin<&'a T>>
    where
        Self: 'a,
        T: 'a;
    /// Either [Pin<&mut T>](core::pin::Pin), `()` or an [Option](core::option::Option) containing [Pin<&mut T>](core::pin::Pin).
    /// 
    /// This is an option if the maybe is run-time managed, otherwise it's the inner type or void if compile-time managed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// use core::pin::Pin;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::PurePinMut<'static>, Option<Pin<&mut i32>>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::PurePinMut<'static>, Pin<&mut i32>);
    /// assert_type_eq_all!(<() as Maybe<i32>>::PurePinMut<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::PurePinMut<'static>, Pin<&mut i32>);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::PurePinMut<'static>, ());
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// 
    /// let mut pure = Maybe::<i32>::pure_pin_mut(maybe.as_mut());
    /// 
    /// assert_eq!(&mut *pure, &mut 777);
    /// ```
    type PurePinMut<'a>: PureMaybe<Pin<&'a mut T>>
    where
        Self: 'a,
        T: 'a;

    /// The same kind of maybe, but containing a different inner value, `U`.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Mapped<u64>, Option<u64>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::Mapped<u64>, u64);
    /// assert_type_eq_all!(<() as Maybe<i32>>::Mapped<u64>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Mapped<u64>, [u64; 1]);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Mapped<u64>, [u64; 0]);
    /// 
    /// let maybe = [666];
    /// 
    /// let mapped = Maybe::<i32>::map(maybe, |x| x + 111);
    /// 
    /// assert_eq!(mapped, [777]);
    /// ```
    type Mapped<U>: Maybe<U>
    where
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    /// The same kind of maybe with the reference of its internal value removed.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<&i32> as Maybe<&i32>>::Copied, Option<i32>);
    /// assert_type_eq_all!(<&i32 as Maybe<&i32>>::Copied, i32);
    /// assert_type_eq_all!(<() as Maybe<&i32>>::Copied, ());
    /// assert_type_eq_all!(<[&i32; 1] as Maybe<&i32>>::Copied, [i32; 1]);
    /// assert_type_eq_all!(<[&i32; 0] as Maybe<&i32>>::Copied, [i32; 0]);
    /// 
    /// // This is supposed to work, but the compiler gets confused...
    /// /*assert_type_eq_all!(<Option<i32> as Maybe<i32>>::Copied, Option<i32>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::Copied, i32);
    /// assert_type_eq_all!(<() as Maybe<i32>>::Copied, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::Copied, [i32; 1]);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::Copied, [i32; 0]);*/
    /// 
    /// let maybe = [777];
    /// let referenced = Maybe::<i32>::as_ref(&maybe);
    /// 
    /// // This is supposed to work, but the compiler gets confused...
    /// //let copy1 = Maybe::<i32>::copied(&maybe);
    /// //assert_eq!(copy1, [777]);
    /// 
    /// let copy2 = Maybe::<&i32>::copied(&referenced);
    /// assert_eq!(copy2, [777]);
    /// ```
    type Copied: Maybe<Copied<T>>
    where
        T: Sized,
        (): StaticMaybe<Copied<T>>;
    
    /// The same kind of maybe, but with its internal value mutably borrowed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsRef<'static>, Option<&i32>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::AsRef<'static>, &i32);
    /// assert_type_eq_all!(<() as Maybe<i32>>::AsRef<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsRef<'static>, [&i32; 1]);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsRef<'static>, [&i32; 0]);
    /// 
    /// let maybe = [777];
    /// 
    /// let referenced = Maybe::<i32>::as_ref(&maybe);
    /// 
    /// assert_eq!(referenced, [&777]);
    /// ```
    type AsRef<'a>: Maybe<&'a T> + 'a = Self::Mapped<&'a T>
    where
        Self: 'a,
        T: 'a;
    /// The same kind of maybe, but with its internal value mutably borrowed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsMut<'static>, Option<&mut i32>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::AsMut<'static>, &mut i32);
    /// assert_type_eq_all!(<() as Maybe<i32>>::AsMut<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsMut<'static>, [&mut i32; 1]);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsMut<'static>, [&mut i32; 0]);
    /// 
    /// let mut maybe = [777];
    /// 
    /// let referenced = Maybe::<i32>::as_mut(&mut maybe);
    /// 
    /// assert_eq!(referenced, [&mut 777]);
    /// ```
    type AsMut<'a>: Maybe<&'a mut T> + 'a = Self::Mapped<&'a mut T>
    where
        Self: 'a,
        T: 'a;
    /// The same kind of maybe, but with its pinned internal value borrowed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// use core::pin::Pin;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsPinRef<'static>, Option<Pin<&i32>>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::AsPinRef<'static>, Pin<&i32>);
    /// assert_type_eq_all!(<() as Maybe<i32>>::AsPinRef<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsPinRef<'static>, [Pin<&i32>; 1]);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsPinRef<'static>, [Pin<&i32>; 0]);
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// 
    /// let referenced = Maybe::<i32>::as_pin_ref(maybe.as_ref());
    /// 
    /// assert_eq!(referenced, [core::pin::pin!(777)]);
    /// ```
    type AsPinRef<'a>: Maybe<Pin<&'a T>> + 'a = Self::Mapped<Pin<&'a T>>
    where
        Self: 'a,
        T: 'a;
    /// The same kind of maybe, but with its pinned internal value borrowed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// use core::pin::Pin;
    /// 
    /// assert_type_eq_all!(<Option<i32> as Maybe<i32>>::AsPinMut<'static>, Option<Pin<&mut i32>>);
    /// assert_type_eq_all!(<i32 as Maybe<i32>>::AsPinMut<'static>, Pin<&mut i32>);
    /// assert_type_eq_all!(<() as Maybe<i32>>::AsPinMut<'static>, ());
    /// assert_type_eq_all!(<[i32; 1] as Maybe<i32>>::AsPinMut<'static>, [Pin<&mut i32>; 1]);
    /// assert_type_eq_all!(<[i32; 0] as Maybe<i32>>::AsPinMut<'static>, [Pin<&mut i32>; 0]);
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// 
    /// let referenced = Maybe::<i32>::as_pin_mut(maybe.as_mut());
    /// 
    /// assert_eq!(referenced, [core::pin::pin!(777)]);
    /// ```
    type AsPinMut<'a>: Maybe<Pin<&'a mut T>> + 'a = Self::Mapped<Pin<&'a mut T>>
    where
        Self: 'a,
        T: 'a;
    /// The same kind of maybe, but with its internal value dereferenced.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<Box<i32>> as Maybe<Box<i32>>>::AsDeref<'static>, Option<&i32>);
    /// assert_type_eq_all!(<Box<i32> as Maybe<Box<i32>>>::AsDeref<'static>, &i32);
    /// assert_type_eq_all!(<() as Maybe<Box<i32>>>::AsDeref<'static>, ());
    /// assert_type_eq_all!(<[Box<i32>; 1] as Maybe<Box<i32>>>::AsDeref<'static>, [&i32; 1]);
    /// assert_type_eq_all!(<[Box<i32>; 0] as Maybe<Box<i32>>>::AsDeref<'static>, [&i32; 0]);
    /// 
    /// let mut maybe = [Box::new(777)];
    /// 
    /// let dereferenced = Maybe::<Box<i32>>::as_deref(&maybe);
    /// 
    /// assert_eq!(dereferenced, [&777]);
    /// ```
    type AsDeref<'a>: Maybe<&'a <T as Deref>::Target> + 'a = Self::Mapped<&'a <T as Deref>::Target>
    where
        Self: 'a,
        T: Deref + 'a;
    /// The same kind of maybe, but with its internal value dereferenced.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// use static_assertions::*;
    /// 
    /// assert_type_eq_all!(<Option<Box<i32>> as Maybe<Box<i32>>>::AsDerefMut<'static>, Option<&mut i32>);
    /// assert_type_eq_all!(<Box<i32> as Maybe<Box<i32>>>::AsDerefMut<'static>, &mut i32);
    /// assert_type_eq_all!(<() as Maybe<Box<i32>>>::AsDerefMut<'static>, ());
    /// assert_type_eq_all!(<[Box<i32>; 1] as Maybe<Box<i32>>>::AsDerefMut<'static>, [&mut i32; 1]);
    /// assert_type_eq_all!(<[Box<i32>; 0] as Maybe<Box<i32>>>::AsDerefMut<'static>, [&mut i32; 0]);
    /// 
    /// let mut maybe = [Box::new(777)];
    /// 
    /// let dereferenced = Maybe::<Box<i32>>::as_deref_mut(&mut maybe);
    /// 
    /// assert_eq!(dereferenced, [&mut 777]);
    /// ```
    type AsDerefMut<'a>: Maybe<&'a mut <T as Deref>::Target> + 'a = Self::Mapped<&'a mut <T as Deref>::Target>
    where
        Self: 'a,
        T: Deref + 'a;

    /// Returns `true` if the maybe contains a value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// assert!(Maybe::<i32>::is_some(&maybe));
    /// assert!(!Maybe::<i32>::is_some(&empty));
    /// ```
    fn is_some(&self) -> bool;
    /// Returns `true` if the maybe does not contain a value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// assert!(!Maybe::<i32>::is_none(&maybe));
    /// assert!(Maybe::<i32>::is_none(&empty));
    /// ```
    fn is_none(&self) -> bool;
    /// Returns a maybe referencing this maybe's internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// 
    /// let referenced = Maybe::<i32>::as_ref(&maybe);
    /// 
    /// assert_eq!(referenced, [&777]);
    /// ```
    fn as_ref<'a>(&'a self) -> Self::AsRef<'a>
    where
        T: 'a;
    /// Returns a maybe mutably referencing this maybe's internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [777];
    /// 
    /// let referenced = Maybe::<i32>::as_mut(&mut maybe);
    /// 
    /// assert_eq!(referenced, [&mut 777]);
    /// ```
    fn as_mut<'a>(&'a mut self) -> Self::AsMut<'a>
    where
        T: 'a;
    /// Returns a maybe referencing this pinned maybe's internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// 
    /// let referenced = Maybe::<i32>::as_pin_ref(maybe.as_ref());
    /// 
    /// assert_eq!(referenced, [core::pin::pin!(777)]);
    /// ```
    fn as_pin_ref<'a>(self: Pin<&'a Self>) -> Self::AsPinRef<'a>
    where
        T: 'a;
    /// Returns a maybe mutably referencing this pinned maybe's internal value.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// 
    /// let referenced = Maybe::<i32>::as_pin_mut(maybe.as_mut());
    /// 
    /// assert_eq!(referenced, [core::pin::pin!(777)]);
    /// ```
    fn as_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::AsPinMut<'a>
    where
        T: 'a;
    /// Returns a slice that contains the internal value, if it exists.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// 
    /// let slice = Maybe::<i32>::as_slice(&maybe);
    /// 
    /// assert_eq!(slice, &[777]);
    /// ```
    fn as_slice(&self) -> &[T]
    where
        T: Sized;
    /// Returns a mutable slice that contains the internal value, if it exists.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [777];
    /// 
    /// let slice = Maybe::<i32>::as_mut_slice(&mut maybe);
    /// 
    /// assert_eq!(slice, &mut [777]);
    /// ```
    fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized;
    /// Returns the internal value, if it exists. Otherwise panics with a user-defined message.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// 
    /// let value = Maybe::<i32>::expect(maybe, "I sure hope this doesn't panic!");
    /// 
    /// assert_eq!(value, 777);
    /// ```
    fn expect(self, msg: &str) -> T
    where
        T: Sized;
    /// Returns the internal value, if it exists. Otherwise panics.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// 
    /// let value = Maybe::<i32>::unwrap(maybe);
    /// 
    /// assert_eq!(value, 777);
    /// ```
    fn unwrap(self) -> T
    where
        T: Sized;
    /// Returns the internal value by reference, if it exists. Otherwise panics.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// 
    /// let value = Maybe::<i32>::unwrap_ref(&maybe);
    /// 
    /// assert_eq!(value, &777);
    /// ```
    fn unwrap_ref(&self) -> &T;
    /// Returns the internal value by mutable reference, if it exists. Otherwise panics.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [777];
    /// 
    /// let value = Maybe::<i32>::unwrap_mut(&mut maybe);
    /// 
    /// assert_eq!(value, &mut 777);
    /// ```
    fn unwrap_mut(&mut self) -> &mut T;
    /// Returns the pinned internal value by reference, if it exists. Otherwise panics.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// 
    /// let value = Maybe::<i32>::unwrap_pin_ref(maybe.as_ref());
    /// 
    /// assert_eq!(value, core::pin::pin!(777));
    /// ```
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a;
    /// Returns the pinned internal value by mutable reference, if it exists. Otherwise panics.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// 
    /// let value = Maybe::<i32>::unwrap_pin_mut(maybe.as_mut());
    /// 
    /// assert_eq!(value, core::pin::pin!(777));
    /// ```
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a;
    /// Returns the internal value, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let value = Maybe::<i32>::unwrap_or(maybe, 666);
    /// let default = Maybe::<i32>::unwrap_or(empty, 666);
    /// 
    /// assert_eq!(value, 777);
    /// assert_eq!(default, 666);
    /// ```
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized;
    /// Returns the internal value by reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let value = Maybe::<i32>::unwrap_ref_or(&maybe, &666);
    /// let default = Maybe::<i32>::unwrap_ref_or(&empty, &666);
    /// 
    /// assert_eq!(value, &777);
    /// assert_eq!(default, &666);
    /// ```
    fn unwrap_ref_or<'a>(&'a self, default: &'a T) -> &'a T
    where
        T: 'a;
    /// Returns the internal value by mutable reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [777];
    /// let mut empty = [];
    /// 
    /// let mut default = 666;
    /// let value = Maybe::<i32>::unwrap_mut_or(&mut maybe, &mut default);
    /// let mut default = 666;
    /// let default = Maybe::<i32>::unwrap_mut_or(&mut empty, &mut default);
    /// 
    /// assert_eq!(value, &mut 777);
    /// assert_eq!(default, &mut 666);
    /// ```
    fn unwrap_mut_or<'a>(&'a mut self, default: &'a mut T) -> &'a mut T
    where
        T: 'a;
    /// Returns the pinned internal value by reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// let empty = core::pin::pin!([]);
    /// 
    /// let default = core::pin::pin!(666);
    /// let value = Maybe::<i32>::unwrap_pin_ref_or(maybe.as_ref(), default.as_ref());
    /// let default = Maybe::<i32>::unwrap_pin_ref_or(empty.as_ref(), default.as_ref());
    /// 
    /// assert_eq!(value, core::pin::pin!(777));
    /// assert_eq!(default, core::pin::pin!(666));
    /// ```
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a;
    /// Returns the pinned internal value by mutable reference, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// let mut empty = core::pin::pin!([]);
    /// 
    /// let default = core::pin::pin!(666);
    /// let value = Maybe::<i32>::unwrap_pin_mut_or(maybe.as_mut(), default);
    /// let default = core::pin::pin!(666);
    /// let default = Maybe::<i32>::unwrap_pin_mut_or(empty.as_mut(), default);
    /// 
    /// assert_eq!(value, core::pin::pin!(777));
    /// assert_eq!(default, core::pin::pin!(666));
    /// ```
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, default: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a;
    /// Returns the internal value, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let value = Maybe::<i32>::unwrap_or_else(maybe, || 666);
    /// let default = Maybe::<i32>::unwrap_or_else(empty, || 666);
    /// 
    /// assert_eq!(value, 777);
    /// assert_eq!(default, 666);
    /// ```
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized;
    /// Returns the internal value by reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let value = Maybe::<i32>::unwrap_ref_or_else(&maybe, || &666);
    /// let default = Maybe::<i32>::unwrap_ref_or_else(&empty, || &666);
    /// 
    /// assert_eq!(value, &777);
    /// assert_eq!(default, &666);
    /// ```
    fn unwrap_ref_or_else<'a, F>(&'a self, default: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a;
    /// Returns the internal value by mutable reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [777];
    /// let mut empty = [];
    /// 
    /// let mut default = 666;
    /// let value = Maybe::<i32>::unwrap_mut_or_else(&mut maybe, || &mut default);
    /// let mut default = 666;
    /// let default = Maybe::<i32>::unwrap_mut_or_else(&mut empty, || &mut default);
    /// 
    /// assert_eq!(value, &mut 777);
    /// assert_eq!(default, &mut 666);
    /// ```
    fn unwrap_mut_or_else<'a, F>(&'a mut self, default: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a;
    /// Returns the pinned internal value by reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// let empty = core::pin::pin!([]);
    /// 
    /// let default = core::pin::pin!(666);
    /// let value = Maybe::<i32>::unwrap_pin_ref_or_else(maybe.as_ref(), || default.as_ref());
    /// let default = Maybe::<i32>::unwrap_pin_ref_or_else(empty.as_ref(), || default.as_ref());
    /// 
    /// assert_eq!(value, core::pin::pin!(777));
    /// assert_eq!(default, core::pin::pin!(666));
    /// ```
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, default: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a;
    /// Returns the pinned internal value by mutable reference, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// let mut empty = core::pin::pin!([]);
    /// 
    /// let default = core::pin::pin!(666);
    /// let value = Maybe::<i32>::unwrap_pin_mut_or_else(maybe.as_mut(), || default);
    /// let default = core::pin::pin!(666);
    /// let default = Maybe::<i32>::unwrap_pin_mut_or_else(empty.as_mut(), || default);
    /// 
    /// assert_eq!(value, core::pin::pin!(777));
    /// assert_eq!(default, core::pin::pin!(666));
    /// ```
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, default: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a;
    /// Returns the internal value, if it exists. Otherwise returns the result of [T::default()](core::default::Default::default).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let value = Maybe::<i32>::unwrap_or_default(maybe);
    /// let default = Maybe::<i32>::unwrap_or_default(empty);
    /// 
    /// assert_eq!(value, 777);
    /// assert_eq!(default, 0);
    /// ```
    fn unwrap_or_default(self) -> T
    where
        T: Sized + Default;
    /// Maps the internal value with a mapping function, if it exists.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [666];
    /// let empty = [];
    /// 
    /// let mapped = Maybe::<i32>::map(maybe, |x| x + 111);
    /// let still_empty = Maybe::<i32>::map(empty, |x| x + 111);
    /// 
    /// assert_eq!(mapped, [777]);
    /// assert_eq!(still_empty, []);
    /// ```
    fn map<U, F>(self, map: F) -> Self::Mapped<U>
    where
        F: FnOnce(T) -> U,
        T: Sized,
        U: StaticMaybe<U>,
        (): StaticMaybe<U>;
    /// Maps the internal value with a mapping function, if it exists. Otherwise returns `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [666];
    /// let empty = [];
    /// 
    /// let mapped = Maybe::<i32>::map_or(maybe, 999, |x| x + 111);
    /// let default = Maybe::<i32>::map_or(empty, 999, |x| x + 111);
    /// 
    /// assert_eq!(mapped, 777);
    /// assert_eq!(default, 999);
    /// ```
    fn map_or<U, F>(self, default: U, map: F) -> U
    where
        F: FnOnce(T) -> U,
        T: Sized;
    /// Maps the internal value with a mapping function, if it exists. Otherwise returns the result of `default`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [666];
    /// let empty = [];
    /// 
    /// let mapped = Maybe::<i32>::map_or_else(maybe, || 999, |x| x + 111);
    /// let default = Maybe::<i32>::map_or_else(empty, || 999, |x| x + 111);
    /// 
    /// assert_eq!(mapped, 777);
    /// assert_eq!(default, 999);
    /// ```
    fn map_or_else<U, D, F>(self, default: D, map: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
        T: Sized;
    /// Returns an [Ok](core::result::Result::Ok) containing the internal value, otherwise returns [Err](core::result::Result::Err) containing `error`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = ["Right"];
    /// let empty = [];
    /// 
    /// let right = Maybe::<&str>::ok_or(maybe, "Wrong");
    /// let wrong = Maybe::<&str>::ok_or(empty, "Wrong");
    /// 
    /// assert_eq!(right, Ok("Right"));
    /// assert_eq!(wrong, Err("Wrong"));
    /// ```
    fn ok_or<E>(self, error: E) -> Result<T, E>
    where
        T: Sized;
    /// Returns an [Ok](core::result::Result::Ok) containing the internal value, otherwise returns [Err](core::result::Result::Err) containing the result of `error`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = ["Right"];
    /// let empty = [];
    /// 
    /// let right = Maybe::<&str>::ok_or_else(maybe, || "Wrong");
    /// let wrong = Maybe::<&str>::ok_or_else(empty, || "Wrong");
    /// 
    /// assert_eq!(right, Ok("Right"));
    /// assert_eq!(wrong, Err("Wrong"));
    /// ```
    fn ok_or_else<E, F>(self, error: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
        T: Sized;
    /// Dereferences the internal value, if it exists, and returns it in a new maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [Box::new(777)];
    /// 
    /// let dereferenced = Maybe::<Box<i32>>::as_deref(&maybe);
    /// 
    /// assert_eq!(dereferenced, [&777]);
    /// ```
    fn as_deref<'a>(&'a self) -> Self::AsDeref<'a>
    where
        T: Deref + 'a;
    /// Mutable dereferences the internal value, if it exists, and returns it in a new maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [Box::new(777)];
    /// 
    /// let dereferenced = Maybe::<Box<i32>>::as_deref_mut(&mut maybe);
    /// 
    /// assert_eq!(dereferenced, [&mut 777]);
    /// ```
    fn as_deref_mut<'a>(&'a mut self) -> Self::AsDerefMut<'a>
    where
        T: DerefMut + 'a;
    /// Returns the last of the two maybes, if both have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = ();
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::and(a, b), ());
    /// 
    /// let a = "First";
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::and(a, b), ());
    /// 
    /// let a = ();
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::and(a, b), ());
    /// 
    /// let a = "First";
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::and(a, b), "Second");
    /// ```
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
        MaybeAnd::and(self.pure(), other.pure())
    }
    /// Maps the value into a different maybe if it exists using a flatmap function.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe1 = "abcdefg";
    /// let maybe2 = "abcdef";
    /// 
    /// let result1 = Maybe::<&str>::and_then::<&str, _>(maybe1, |value| if value.len() > 6 {None} else {Some(value)});
    /// let result2 = Maybe::<&str>::and_then::<&str, _>(maybe2, |value| if value.len() > 6 {None} else {Some(value)});
    /// 
    /// assert_eq!(result1, None);
    /// assert_eq!(result2, Some("abcdef"));
    /// ```
    #[doc(alias = "flatmap")]
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
        MaybeAndThen::and_then(self.pure(), |x| and_then(x).pure())
    }
    /// Filters the internal value depending on a predicate.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe1 = "abcdefg";
    /// let maybe2 = "abcdef";
    /// 
    /// let result1 = Maybe::<&str>::filter(maybe1, |value| value.len() <= 6);
    /// let result2 = Maybe::<&str>::filter(maybe2, |value| value.len() <= 6);
    /// 
    /// assert_eq!(result1, None);
    /// assert_eq!(result2, Some("abcdef"));
    /// ```
    fn filter<F>(self, predicate: F) -> <Self::Pure as MaybeFilter<T>>::Output
    where
        Self: Sized,
        F: Fn(&T) -> bool,
        Self::Pure: MaybeFilter<T> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
    {
        MaybeFilter::filter(self.pure(), predicate)
    }
    /// Returns the first of the two maybes, if any of them have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = ();
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::or(a, b), ());
    /// 
    /// let a = "First";
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::or(a, b), "First");
    /// 
    /// let a = ();
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::or(a, b), "Second");
    /// 
    /// let a = "First";
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::or(a, b), "First");
    /// ```
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
        MaybeOr::or(self.pure(), other.pure())
    }
    /// Returns the first of the two maybes, if any of them have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = ();
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::or_else(a, || b), ());
    /// 
    /// let a = "First";
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::or_else(a, || b), "First");
    /// 
    /// let a = ();
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::or_else(a, || b), "Second");
    /// 
    /// let a = "First";
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::or_else(a, || b), "First");
    /// ```
    fn or_else<F>(self, or_else: F) -> <Self::Pure as MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure>>::Output
    where
        F: FnOnce<(), Output: Maybe<T, Pure: Sized>>,
        Self: Sized,
        Self::Pure: MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure> + Sized,
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        <Self::Pure as MaybeOr<T, <<F as FnOnce<()>>::Output as Maybe<T>>::Pure>>::Output: Sized
    {
        MaybeOr::or_else(self.pure(), || or_else().pure())
    }
    /// Returns the first of the two maybes, if exactly one of them have a value, otherwise returns an empty maybe.
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let a = ();
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::xor(a, b), ());
    /// 
    /// let a = "First";
    /// let b = ();
    /// 
    /// assert_eq!(Maybe::<&str>::xor(a, b), "First");
    /// 
    /// let a = ();
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::xor(a, b), "Second");
    /// 
    /// let a = "First";
    /// let b = "Second";
    /// 
    /// assert_eq!(Maybe::<&str>::xor(a, b), ());
    /// ```
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
        MaybeXor::xor(self.pure(), other.pure())
    }
    /// Copies the internal value, if it exists, and returns it in a new maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [&777];
    /// 
    /// let copied = Maybe::<&i32>::copied(&maybe);
    /// 
    /// assert_eq!(copied, [777]);
    /// ```
    fn copied(&self) -> Self::Copied
    where
        Copied<T>: Copy,
        T: Sized,
        (): StaticMaybe<Copied<T>>;
    /// Clones the internal value, if it exists, and returns it in a new maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [&Box::new(777)];
    /// 
    /// let cloned = Maybe::<&Box<i32>>::cloned(&maybe);
    /// 
    /// assert_eq!(cloned, [Box::new(777)]);
    /// ```
    fn cloned(&self) -> Self::Copied
    where
        Copied<T>: Clone,
        T: Sized,
        (): StaticMaybe<Copied<T>>;

    /// Converts this maybe into an [Option](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = 777;
    /// let empty = ();
    /// 
    /// let some = Maybe::<i32>::option(maybe);
    /// let none = Maybe::<i32>::option(empty);
    /// 
    /// assert_eq!(some, Some(777));
    /// assert_eq!(none, None);
    /// ```
    fn option(self) -> Option<T>
    where
        T: Sized;
    /// Retrieves the internal value in the form of an [Option](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = 777;
    /// let empty = ();
    /// 
    /// let some = Maybe::<i32>::option_ref(&maybe);
    /// let none = Maybe::<i32>::option_ref(&empty);
    /// 
    /// assert_eq!(some, Some(&777));
    /// assert_eq!(none, None);
    /// ```
    fn option_ref(&self) -> Option<&T>;
    /// Mutably retrieves the internal value in the form of an [Option](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = 777;
    /// let mut empty = ();
    /// 
    /// let some = Maybe::<i32>::option_mut(&mut maybe);
    /// let none = Maybe::<i32>::option_mut(&mut empty);
    /// 
    /// assert_eq!(some, Some(&mut 777));
    /// assert_eq!(none, None);
    /// ```
    fn option_mut(&mut self) -> Option<&mut T>;
    /// Retrieves the pinned internal value in the form of an [Option](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!(777);
    /// let empty = core::pin::pin!(());
    /// 
    /// let some = Maybe::<i32>::option_pin_ref(maybe.as_ref());
    /// let none = Maybe::<i32>::option_pin_ref(empty.as_ref());
    /// 
    /// assert_eq!(some, Some(maybe.as_ref()));
    /// assert_eq!(none, None);
    /// ```
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>;
    /// Mutably retrieves the pinned internal value in the form of an [Option](core::option::Option).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!(777);
    /// let mut empty = core::pin::pin!(());
    /// 
    /// let some = Maybe::<i32>::option_pin_mut(maybe.as_mut());
    /// let none = Maybe::<i32>::option_pin_mut(empty.as_mut());
    /// 
    /// assert_eq!(some, Some(core::pin::pin!(777)));
    /// assert_eq!(none, None);
    /// ```
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>;

    /// Converts the maybe into a pure maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let pure_maybe = Maybe::<i32>::pure(maybe);
    /// let pure_empty = Maybe::<i32>::pure(empty);
    /// 
    /// assert_eq!(pure_maybe, 777);
    /// assert_eq!(pure_empty, ());
    /// ```
    fn pure(self) -> Self::Pure
    where
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self::Pure: Sized;
    /// Retrieves the internal value in a pure maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = [777];
    /// let empty = [];
    /// 
    /// let pure_maybe = Maybe::<i32>::pure_ref(&maybe);
    /// let pure_empty = Maybe::<i32>::pure_ref(&empty);
    /// 
    /// assert_eq!(pure_maybe, &777);
    /// assert_eq!(pure_empty, ());
    /// ```
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a;
    /// Retrieves the internal value in a pure maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = [777];
    /// let mut empty = [];
    /// 
    /// let pure_maybe = Maybe::<i32>::pure_mut(&mut maybe);
    /// let pure_empty = Maybe::<i32>::pure_mut(&mut empty);
    /// 
    /// assert_eq!(pure_maybe, &mut 777);
    /// assert_eq!(pure_empty, ());
    /// ```
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a;
    /// Retrieves the pinned internal value in a pure maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let maybe = core::pin::pin!([777]);
    /// let empty = core::pin::pin!([]);
    /// 
    /// let pure_maybe = Maybe::<i32>::pure_pin_ref(maybe.as_ref());
    /// let pure_empty = Maybe::<i32>::pure_pin_ref(empty.as_ref());
    /// 
    /// assert_eq!(pure_maybe, core::pin::pin!(777));
    /// assert_eq!(pure_empty, ());
    /// ```
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a;
    /// Mutably retrieves the pinned internal value in a pure maybe.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use option_trait::*;
    /// 
    /// let mut maybe = core::pin::pin!([777]);
    /// let mut empty = core::pin::pin!([]);
    /// 
    /// let pure_maybe = Maybe::<i32>::pure_pin_mut(maybe.as_mut());
    /// let pure_empty = Maybe::<i32>::pure_pin_mut(empty.as_mut());
    /// 
    /// assert_eq!(pure_maybe, core::pin::pin!(777));
    /// assert_eq!(pure_empty, ());
    /// ```
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
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
    fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        core::slice::from_ref(self)
    }
    fn as_mut_slice(&mut self) -> &mut [T]
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
    fn unwrap_ref(&self) -> &T
    {
        self
    }
    fn unwrap_mut(&mut self) -> &mut T
    {
        self
    }
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a
    {
        self
    }
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self
    }
    fn unwrap_or(self, _: T) -> T
    where
        T: Sized
    {
        self
    }
    fn unwrap_ref_or<'a>(&'a self, _: &'a T) -> &'a T
    where
        T: 'a
    {
        self
    }
    fn unwrap_mut_or<'a>(&'a mut self, _: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        self
    }
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, _: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        self
    }
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, _: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self
    }
    fn unwrap_or_else<F>(self, _: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        self
    }
    fn unwrap_ref_or_else<'a, F>(&'a self, _: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        self
    }
    fn unwrap_mut_or_else<'a, F>(&'a mut self, _: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        self
    }
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, _: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        self
    }
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, _: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
    {
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
    fn option_ref(&self) -> Option<&T>
    {
        Some(self)
    }
    fn option_mut(&mut self) -> Option<&mut T>
    {
        Some(self)
    }
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        Some(self)
    }
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        Some(self)
    }

    fn pure(self) -> Self::Pure
    where
        T: StaticMaybe<T> + Sized,
        (): StaticMaybe<T>,
        Self::Pure: Sized
    {
        crate::assume_same(self)
    }
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        self
    }
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        self
    }
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        self
    }
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
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
    fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        &[]
    }
    fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        &mut []
    }
    fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        crate::on_unwrap_empty_msg(msg)
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_ref(&self) -> &T
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_mut(&mut self) -> &mut T
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        default
    }
    fn unwrap_ref_or<'a>(&'a self, default: &'a T) -> &'a T
    where
        T: 'a
    {
        default
    }
    fn unwrap_mut_or<'a>(&'a mut self, default: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        default
    }
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        default
    }
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, default: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
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
    fn unwrap_ref_or_else<'a, F>(&'a self, default: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        default()
    }
    fn unwrap_mut_or_else<'a, F>(&'a mut self, default: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        default()
    }
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, default: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        default()
    }
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, default: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
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
    fn option_ref(&self) -> Option<&T>
    {
        None
    }
    fn option_mut(&mut self) -> Option<&mut T>
    {
        None
    }
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        None
    }
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        None
    }

    fn pure(self) -> Self::Pure
    {

    }
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {

    }
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {

    }
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {

    }
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
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
    fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        self.as_slice()
    }
    fn as_mut_slice(&mut self) -> &mut [T]
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
    fn unwrap_ref(&self) -> &T
    {
        self.as_ref().unwrap()
    }
    fn unwrap_mut(&mut self) -> &mut T
    {
        self.as_mut().unwrap()
    }
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a
    {
        self.as_pin_ref().unwrap()
    }
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self.as_pin_mut().unwrap()
    }
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        self.unwrap_or(default)
    }
    fn unwrap_ref_or<'a>(&'a self, default: &'a T) -> &'a T
    where
        T: 'a
    {
        self.as_ref().unwrap_or(default)
    }
    fn unwrap_mut_or<'a>(&'a mut self, default: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        self.as_mut().unwrap_or(default)
    }
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        self.as_pin_ref().unwrap_or(default)
    }
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, default: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self.as_pin_mut().unwrap_or(default)
    }
    fn unwrap_or_else<F>(self, default: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        self.unwrap_or_else(default)
    }
    fn unwrap_ref_or_else<'a, F>(&'a self, default: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        self.as_ref().unwrap_or_else(default)
    }
    fn unwrap_mut_or_else<'a, F>(&'a mut self, default: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        self.as_mut().unwrap_or_else(default)
    }
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, default: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        self.as_pin_ref().unwrap_or_else(default)
    }
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, default: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
    {
        self.as_pin_mut().unwrap_or_else(default)
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
    fn option_ref(&self) -> Option<&T>
    {
        self.as_ref()
    }
    fn option_mut(&mut self) -> Option<&mut T>
    {
        self.as_mut()
    }
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        self.as_pin_ref()
    }
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        self.as_pin_mut()
    }

    fn pure(self) -> Self::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>
    {
        self
    }
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        self.as_ref()
    }
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        self.as_mut()
    }
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        self.as_pin_ref()
    }
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
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
    fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        &[]
    }
    fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Sized
    {
        &mut []
    }
    fn expect(self, msg: &str) -> T
    where
        T: Sized
    {
        crate::on_unwrap_empty_msg(msg)
    }
    fn unwrap(self) -> T
    where
        T: Sized
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_ref(&self) -> &T
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_mut(&mut self) -> &mut T
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        crate::on_unwrap_empty()
    }
    fn unwrap_or(self, default: T) -> T
    where
        T: Sized
    {
        default
    }
    fn unwrap_ref_or<'a>(&'a self, default: &'a T) -> &'a T
    where
        T: 'a
    {
        default
    }
    fn unwrap_mut_or<'a>(&'a mut self, default: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        default
    }
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, default: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        default
    }
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, default: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
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
    fn unwrap_ref_or_else<'a, F>(&'a self, default: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        default()
    }
    fn unwrap_mut_or_else<'a, F>(&'a mut self, default: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        default()
    }
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, default: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        default()
    }
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, default: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
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
    fn option_ref(&self) -> Option<&T>
    {
        None
    }
    fn option_mut(&mut self) -> Option<&mut T>
    {
        None
    }
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        None
    }
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
    {
        None
    }

    fn pure(self) -> Self::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>,
        Self::Pure: Sized
    {
        crate::assume_same(())
    }
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {

    }
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {

    }
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {

    }
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
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
    fn as_slice(&self) -> &[T]
    where
        T: Sized
    {
        self
    }
    fn as_mut_slice(&mut self) -> &mut [T]
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
        let [value] = self;
        value
    }
    fn unwrap_ref(&self) -> &T
    {
        &self[0]
    }
    fn unwrap_mut(&mut self) -> &mut T
    {
        &mut self[0]
    }
    fn unwrap_pin_ref<'a>(self: Pin<&'a Self>) -> Pin<&'a T>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked(|this| this.unwrap_ref())
        }
    }
    fn unwrap_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked_mut(|this| this.unwrap_mut())
        }
    }
    fn unwrap_or(self, _: T) -> T
    where
        T: Sized
    {
        self.unwrap()
    }
    fn unwrap_ref_or<'a>(&'a self, _: &'a T) -> &'a T
    where
        T: 'a
    {
        self.unwrap_ref()
    }
    fn unwrap_mut_or<'a>(&'a mut self, _: &'a mut T) -> &'a mut T
    where
        T: 'a
    {
        self.unwrap_mut()
    }
    fn unwrap_pin_ref_or<'a>(self: Pin<&'a Self>, _: Pin<&'a T>) -> Pin<&'a T>
    where
        T: 'a
    {
        self.unwrap_pin_ref()
    }
    fn unwrap_pin_mut_or<'a>(self: Pin<&'a mut Self>, _: Pin<&'a mut T>) -> Pin<&'a mut T>
    where
        T: 'a
    {
        self.unwrap_pin_mut()
    }
    fn unwrap_or_else<F>(self, _: F) -> T
    where
        F: FnOnce() -> T,
        T: Sized
    {
        self.unwrap()
    }
    fn unwrap_ref_or_else<'a, F>(&'a self, _: F) -> &'a T
    where
        F: FnOnce() -> &'a T,
        T: 'a
    {
        self.unwrap_ref()
    }
    fn unwrap_mut_or_else<'a, F>(&'a mut self, _: F) -> &'a mut T
    where
        F: FnOnce() -> &'a mut T,
        T: 'a
    {
        self.unwrap_mut()
    }
    fn unwrap_pin_ref_or_else<'a, F>(self: Pin<&'a Self>, _: F) -> Pin<&'a T>
    where
        F: FnOnce() -> Pin<&'a T>,
        T: 'a
    {
        self.unwrap_pin_ref()
    }
    fn unwrap_pin_mut_or_else<'a, F>(self: Pin<&'a mut Self>, _: F) -> Pin<&'a mut T>
    where
        F: FnOnce() -> Pin<&'a mut T>,
        T: 'a
    {
        self.unwrap_pin_mut()
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
    fn option_ref(&self) -> Option<&T>
    {
        Some(&self[0])
    }
    fn option_mut(&mut self) -> Option<&mut T>
    {
        Some(&mut self[0])
    }
    fn option_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>
    {
        Some(unsafe {
            self.map_unchecked(|this| &this[0])
        })
    }
    fn option_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>
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

    fn pure(self) -> Self::Pure
    where
        T: StaticMaybe<T>,
        (): StaticMaybe<T>
    {
        self.unwrap()
    }
    fn pure_ref<'a>(&'a self) -> Self::PureRef<'a>
    where
        T: 'a
    {
        &self[0]
    }
    fn pure_mut<'a>(&'a mut self) -> Self::PureMut<'a>
    where
        T: 'a
    {
        &mut self[0]
    }
    fn pure_pin_ref<'a>(self: Pin<&'a Self>) -> Self::PurePinRef<'a>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked(|this| &this[0])
        }
    }
    fn pure_pin_mut<'a>(self: Pin<&'a mut Self>) -> Self::PurePinMut<'a>
    where
        T: 'a
    {
        unsafe {
            self.map_unchecked_mut(|this| &mut this[0])
        }
    }
}