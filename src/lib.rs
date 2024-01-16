#![allow(async_fn_in_trait)]

use thiserror::Error;

pub use byte_array::*;

mod byte_array;
pub mod i2c;

#[derive(Clone, Copy, Debug, Error)]
pub enum ReadRegisterError<B, D> {
    BusError(B),
    DeserializationError(D),
}

#[derive(Clone, Copy, Debug, Error)]
pub enum WriteRegisterError<B, S> {
    BusError(B),
    SerializationError(S),
}

/// The generic top level trait for all register values
pub trait Register {
    fn id() -> u8;
}

pub trait ReadableRegister: Register + FromByteArray {
    #[inline]
    fn readable_id() -> u8 {
        Self::id() | 0x40
    }
}

pub trait WritableRegister: Register + ToByteArray {
    #[inline]
    fn writeable_id() -> u8 {
        Self::id()
    }
}
