use crate::{NotVoid, PureMaybe, Maybe};

pub trait MaybeFilter<T>: PureMaybe<T>
{
    type Output: PureMaybe<T>;

    fn filter<F>(self, predicate: F) -> Self::Output
    where
        F: FnOnce(&T) -> bool;
}

impl<T, Lhs> MaybeFilter<T> for Lhs
where
    Lhs: PureMaybe<T>
{
    default type Output = Option<T>;

    default fn filter<F>(self, predicate: F) -> Self::Output
    where
        F: FnOnce(&T) -> bool
    {
        crate::assume_same(self.option().filter(predicate))
    }
}

impl<T> MaybeFilter<T> for Option<T>
{
    type Output = Option<T>;

    fn filter<F>(self, predicate: F) -> Self::Output
    where
        F: FnOnce(&T) -> bool
    {
        self.filter(predicate)
    }
}

impl<T> MaybeFilter<T> for ()
where
    T: NotVoid
{
    type Output = ();

    fn filter<F>(self, _: F) -> Self::Output
    {
        
    }
}

impl<T> MaybeFilter<T> for T
{
    type Output = Option<T>;

    fn filter<F>(self, predicate: F) -> Self::Output
    where
        F: FnOnce(&T) -> bool
    {
        self.option().filter(predicate)
    }
}