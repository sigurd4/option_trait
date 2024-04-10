use crate::{private, NotVoid};

pub trait Maybe<T>: private::Maybe<T>
where
    T: ?Sized
{
    fn into_option(self) -> Option<T>
    where
        T: Sized;
    fn as_option(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
}
impl<Some> const Maybe<Some> for Some
where
    Some: ?Sized
{
    fn into_option(self) -> Option<Some>
    where
        Some: Sized
    {
        Some(self)
    }
    fn as_option(&self) -> Option<&Some>
    {
        Some(self)
    }
    fn as_option_mut(&mut self) -> Option<&mut Some>
    {
        Some(self)
    }
}
impl<Some> const Maybe<Some> for ()
where
    Some: NotVoid + ?Sized
{
    fn into_option(self) -> Option<Some>
    where
        Some: Sized
    {
        None
    }
    fn as_option(&self) -> Option<&Some>
    {
        None
    }
    fn as_option_mut(&mut self) -> Option<&mut Some>
    {
        None
    }
}
impl<Some> const Maybe<Some> for Option<Some>
{
    fn into_option(self) -> Option<Some>
    {
        self
    }
    fn as_option(&self) -> Option<&Some>
    {
        self.as_ref()
    }
    fn as_option_mut(&mut self) -> Option<&mut Some>
    {
        self.as_mut()
    }
}