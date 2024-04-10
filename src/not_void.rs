use crate::private;

pub trait NotVoid: private::NotVoid
{

}

impl<T> NotVoid for T
where
    T: private::NotVoid + ?Sized
{

}