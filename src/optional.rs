use crate::{Maybe, OptionObj};

pub trait Optional: OptionObj + Maybe<Self::Some>
{
    type Some;
    fn some(some: Self::Some) -> Self;
    fn none() -> Self;
}
impl<Some> const Optional for Option<Some>
{
    type Some = Some;
    fn some(some: <Option<Some> as Optional>::Some) -> Self
    {
        Some(some)
    }
    fn none() -> Self
    {
        None
    }
}