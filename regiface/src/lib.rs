//! ## Overview
//!
//! This crate provides a handful of utility types for writing abstractions for interfacing with
//! register based devices. Most commonly, this would be utilized when writing drivers for
//! external peripherals within an embedded environment. As such, some utility functions
//! are provided for reading and writing registers on devices across I2C or SPI buses.
//!
//! This crate provides two core traits:
//! - [`Register`] for types that represent a value stored within an addressable register
//! - [`Command`] for types that represent an invokable command with parameters and response
//!
//! ### Readable Registers
//!
//! A register in which values can be retrieved from, or read from, is represented as any type that
//! implements the [`ReadableRegister`] trait. This trait is very little more than just a marker trait,
//! but it represents a type that is both a [`Register`] and that can be created from a byte array through
//! the [`FromByteArray`] trait. The bulk of the work in writing a type that can be read from a register
//! will be in implementing the [`FromByteArray`] trait.
//!
//! A type that implements the [`ReadableRegister`] trait can then be used with provided utility methods
//! such as those provided by the [`i2c`] or [`spi`] modules.
//!
//! #### Register Implementation Example
//!
//! ```
//! use regiface::{register, ReadableRegister, FromByteArray};
//!
//! #[register(42u8)]
//! #[derive(ReadableRegister, Debug)]
//! pub struct MyRegister {
//!     value: u8
//! }
//!
//! impl FromByteArray for MyRegister {
//!     type Error = core::convert::Infallible;
//!     type Array = [u8; 1];
//!
//!     fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
//!         Ok(Self {
//!             value: bytes[0]
//!         })
//!     }
//! }
//! ```
//!
//! ### Writable Registers
//!
//! A register in which values can be written to is represented as any type that
//! implements the [`WritableRegister`] trait. This trait is very little more than just a marker trait,
//! but it represents a type that is both a [`Register`] and that can be serialized into a byte array through
//! the [`ToByteArray`] trait. The bulk of the work in writing a type that can be written to a register
//! will be in implementing the [`ToByteArray`] trait.
//!
//! A type that implements the [`WritableRegister`] trait can then be used with provided utility methods
//! such as those provided by the [`i2c`] or [`spi`] modules.
//!
//! #### Register Implementation Example
//!
//! ```
//! use regiface::{register, WritableRegister, ToByteArray};
//!
//! #[register(42u8)]
//! #[derive(WritableRegister, Debug)]
//! pub struct MyRegister {
//!     value: u8
//! }
//!
//! impl ToByteArray for MyRegister {
//!     type Error = core::convert::Infallible;
//!     type Array = [u8; 1];
//!
//!     fn to_bytes(self) -> Result<Self::Array, Self::Error> {
//!         Ok([self.value])
//!     }
//! }
//! ```
//!
//! ### Commands
//!
//! A command represents an invokable action with optional parameters and response. Commands are
//! implemented using the [`Command`] trait, which specifies both the command parameters and expected
//! response type. For commands or responses without parameters, the [`NoParameters`] type can be used.
//!
//! #### Command Implementation Example
//!
//! ```rust
//! use regiface::{Command, ToByteArray, FromByteArray, NoParameters};
//!
//! struct GetTemperature;
//!
//! impl Command for GetTemperature {
//!     type IdType = u8;
//!     type CommandParameters = NoParameters;
//!     type ResponseParameters = Temperature;
//!
//!     fn id() -> Self::IdType {
//!         0x42
//!     }
//!
//!     fn invoking_parameters(self) -> Self::CommandParameters {
//!         NoParameters::default()
//!     }
//! }
//!
//! struct Temperature {
//!     celsius: f32
//! }
//!
//! impl FromByteArray for Temperature {
//!     type Error = core::convert::Infallible;
//!     type Array = [u8; 4];
//!
//!     fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
//!         let celsius = f32::from_be_bytes(bytes);
//!         Ok(Self { celsius })
//!     }
//! }
//! ```

pub use byte_array::{FromByteArray, ToByteArray};
pub use command::*;
pub use regiface_macros::{register, ReadableRegister, WritableRegister};
pub use register::*;

pub mod byte_array;
mod command;
pub mod errors;
pub mod i2c;
pub mod id;
mod register;
pub mod spi;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Default)]
pub struct NoParameters {}
