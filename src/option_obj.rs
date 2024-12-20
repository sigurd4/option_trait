use crate::{private, OptionKind};

#[const_trait]
pub trait OptionObj: private::Optional
{
    fn kind(&self) -> OptionKind
    {
        if self.is_some()
        {
            OptionKind::Some
        }
        else
        {
            OptionKind::None
        }
    }
    fn is_some(&self) -> bool;
    fn is_none(&self) -> bool
    {
        !self.is_some()
    }
}
impl<Some> const OptionObj for Option<Some>
{
    fn is_some(&self) -> bool
    {
        Option::is_some(&self)
    }
}