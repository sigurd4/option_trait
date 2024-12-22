use crate::{NotVoid, PureMaybe, StaticMaybe};

/// A sealed trait for specifically either `T`, `()`.
pub trait PureStaticMaybe<T>: PureMaybe<T> + StaticMaybe<T>
where
    T: ?Sized
{

}
impl<T> PureStaticMaybe<T> for T
where
    T: ?Sized
{

}
impl<T> PureStaticMaybe<T> for ()
where
    T: NotVoid + ?Sized
{

}