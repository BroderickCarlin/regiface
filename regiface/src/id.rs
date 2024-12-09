use std::convert::Infallible;

use crate::ToByteArray;

pub trait Id: ToByteArray<Error = Infallible> {}

impl Id for u8 {}
impl Id for u16 {}
impl Id for u32 {}
impl Id for u64 {}
impl Id for u128 {}
