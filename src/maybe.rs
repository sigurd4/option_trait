use crate::private;

pub trait Maybe<T>: private::Maybe<T>
{
    fn into_option(self) -> Option<T>;
    fn as_option(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
}
impl<Some> const Maybe<Some> for Some
{
    fn into_option(self) -> Option<Some>
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
    Some: private::NotVoid
{
    fn into_option(self) -> Option<Some>
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