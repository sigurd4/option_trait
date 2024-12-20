use crate::{private, Maybe, NotVoid};

pub trait PureMaybe<T>: Maybe<T> + private::PureMaybe<T>
where
    T: ?Sized
{

}

impl<T> PureMaybe<T> for Option<T>
{

}
impl<T> PureMaybe<T> for T
where
    T: ?Sized
{

}
impl<T> PureMaybe<T> for ()
where
    T: NotVoid + ?Sized
{

}