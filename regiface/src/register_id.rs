use std::convert::Infallible;

use crate::ToByteArray;

pub trait RegisterId: ToByteArray<Error = Infallible> {}

impl RegisterId for u8 {}
impl RegisterId for u16 {}
impl RegisterId for u32 {}
impl RegisterId for u64 {}
impl RegisterId for u128 {}
