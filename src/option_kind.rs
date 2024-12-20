use core::marker::ConstParamTy;


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, ConstParamTy)]
pub enum OptionKind
{
    None = 0,
    Some = 1
}

impl<Some> /*const*/ From<&Option<Some>> for OptionKind
{
    fn from(option: &Option<Some>) -> Self
    {
        if option.is_some()
        {
            OptionKind::Some
        }
        else
        {
            OptionKind::None
        }
    }
}