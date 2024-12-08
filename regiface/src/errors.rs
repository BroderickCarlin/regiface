use thiserror::Error;

use crate::{FromByteArray, ToByteArray};

/// A type alias to represent an error that can be encountered when reading or writing a register
/// that implements both [`ReadableRegister`](crate::ReadableRegister) and [`WritableRegister`](crate::WritableRegister)
pub type RegisterError<B, R> = Error<B, <R as FromByteArray>::Error, <R as ToByteArray>::Error>;
/// A type alias to represent an error that can be encountered when reading or writing any register
/// that implements the [`PackedStruct`](packed_struct::PackedStruct) trait
#[cfg(feature = "packed_struct")]
pub type PackedRegisterError<B> =
    Error<B, packed_struct::prelude::PackingError, packed_struct::prelude::PackingError>;

/// The top-level error type representing any error that could be encountered when reading or writing a
/// register value.
///
/// The [`RegisterError`] and [`PackedRegisterError`] type aliases may assist in specifying the correct
/// generic types.
pub enum Error<B, D, S> {
    BusError(B),
    DeserializationError(D),
    SerializationError(S),
}

/// A type alias to represent the possible results of reading a specific register value
pub type ReadRegisterResult<B, R> = Result<R, ReadRegisterError<B, <R as FromByteArray>::Error>>;

/// A type alias to represent the possible results of writing a specific register value
pub type WriteRegisterResult<B, R> = Result<(), WriteRegisterError<B, <R as ToByteArray>::Error>>;

/// An error that can be encountered when reading a register value.
#[derive(Clone, Copy, Debug, Error)]
pub enum ReadRegisterError<B, D> {
    BusError(B),
    DeserializationError(D),
}

/// An error that can be encountered when writing a register value.
#[derive(Clone, Copy, Debug, Error)]
pub enum WriteRegisterError<B, S> {
    BusError(B),
    SerializationError(S),
}

impl<B, D, S> From<ReadRegisterError<B, D>> for Error<B, D, S> {
    fn from(value: ReadRegisterError<B, D>) -> Self {
        match value {
            ReadRegisterError::BusError(e) => Error::BusError(e),
            ReadRegisterError::DeserializationError(e) => Error::DeserializationError(e),
        }
    }
}

impl<B, D, S> From<WriteRegisterError<B, S>> for Error<B, D, S> {
    fn from(value: WriteRegisterError<B, S>) -> Self {
        match value {
            WriteRegisterError::BusError(e) => Error::BusError(e),
            WriteRegisterError::SerializationError(e) => Error::SerializationError(e),
        }
    }
}
