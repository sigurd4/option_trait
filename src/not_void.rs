use crate::private;

pub auto trait NotVoid
{

}

impl !NotVoid for ()
{

}