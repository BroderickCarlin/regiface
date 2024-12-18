//! Error types for register interface operations.
//!
//! This module provides a two-tier error handling system:
//! 1. Specific error types that preserve detailed error information including the underlying error types
//! 2. A generic [`Error`] type that can represent any error case but forgoes the specific error details
//!
//! This approach allows for detailed error handling when needed while also providing a simpler,
//! unified error type when the specific details aren't required.

/// Error that can occur when reading from a register.
///
/// Generic over the bus error type `B` and deserialization error type `D`.
#[derive(Clone, Copy, Debug)]
pub enum ReadRegisterError<B, D> {
    /// An error occurred while communicating over the bus
    BusError(B),
    /// An error occurred while deserializing the received data
    DeserializationError(D),
}

/// Error that can occur when writing to a register.
///
/// Generic over the bus error type `B` and serialization error type `S`.
#[derive(Clone, Copy, Debug)]
pub enum WriteRegisterError<B, S> {
    /// An error occurred while communicating over the bus
    BusError(B),
    /// An error occurred while serializing the data to be sent
    SerializationError(S),
}

/// Error that can occur when executing a command.
///
/// Generic over the bus error type `B`, serialization error type `S`,
/// and deserialization error type `D`.
#[derive(Clone, Copy, Debug)]
pub enum CommandError<B, S, D> {
    /// An error occurred while communicating over the bus
    BusError(B),
    /// An error occurred while serializing the command data
    SerializationError(S),
    /// An error occurred while deserializing the command response
    DeserializationError(D),
}

/// A simplified error type that represents any error that can occur during register operations.
///
/// This type intentionally discards the specific error details in favor of a simpler,
/// unified error type. Use the specific error types ([`ReadRegisterError`], [`WriteRegisterError`],
/// [`CommandError`]) when you need access to the underlying error information.
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// An error occurred while communicating over the bus
    BusError,
    /// An error occurred during data serialization
    SerializationError,
    /// An error occurred during data deserialization
    DeserializationError,
}

impl<B, D> From<ReadRegisterError<B, D>> for Error {
    fn from(value: ReadRegisterError<B, D>) -> Self {
        match value {
            ReadRegisterError::BusError(_) => Self::BusError,
            ReadRegisterError::DeserializationError(_) => Self::DeserializationError,
        }
    }
}

impl<B, S> From<WriteRegisterError<B, S>> for Error {
    fn from(value: WriteRegisterError<B, S>) -> Self {
        match value {
            WriteRegisterError::BusError(_) => Self::BusError,
            WriteRegisterError::SerializationError(_) => Self::SerializationError,
        }
    }
}

impl<B, D, S> From<CommandError<B, D, S>> for Error {
    fn from(value: CommandError<B, D, S>) -> Self {
        match value {
            CommandError::BusError(_) => Self::BusError,
            CommandError::DeserializationError(_) => Self::DeserializationError,
            CommandError::SerializationError(_) => Self::SerializationError,
        }
    }
}
