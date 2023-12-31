#![no_std]

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(auto_traits)]
#![feature(negative_impls)]

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum OptionKind
{
    None = 0,
    Some = 1
}

mod private
{
    pub trait Optional {}
    impl<T> Optional for Option<T> {}

    pub auto trait NotVoid {}
    impl !NotVoid for () {}

    pub trait Maybe<T> {}
    impl<T> Maybe<T> for Option<T> {}
    impl<T> Maybe<T> for T {}
    impl<T> Maybe<T> for ()
    where
        T: NotVoid {}
}

#[const_trait]
pub trait Maybe<T>: private::Maybe<T>
{
    fn into_option(self) -> Option<T>;
}
impl<Some> const Maybe<Some> for Some
{
    fn into_option(self) -> Option<Some>
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
}
impl<Some> const Maybe<Some> for Option<Some>
{
    fn into_option(self) -> Option<Some>
    {
        self
    }
}

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
    fn is_none(&self) -> bool;
}
impl<Some> const From<&Option<Some>> for OptionKind
{
    fn from(option: &Option<Some>) -> Self
    {
        option.kind()
    }
}
impl<Some> const OptionObj for Option<Some>
{
    fn is_some(&self) -> bool
    {
        Option::is_some(&self)
    }
    fn is_none(&self) -> bool
    {
        !self.is_some()
    }
}

#[const_trait]
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

pub trait Some<const OPTION: OptionKind>: Optional {}
impl<O> Some<{OptionKind::Some}> for O
where
    O: Optional {}

pub trait None<const OPTION: OptionKind>: Optional {}
impl<O> None<{OptionKind::None}> for O
where
    O: Optional {}