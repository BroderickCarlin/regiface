//! ## Overview
//!
//! This crate provides a handful of utility types for writing abstractions for interfacing with
//! register based devices. Most commonly, this would be utilized when writing drivers for
//! external peripherals within an embedded environment. As such, some utility functions
//! are provided for reading and writing registers on devices across I2C or SPI buses.
//!
//! This crate provides a single trait to be implemented by all types that represent a value that
//! is stored within an addressable register, aptly named [`Register`]. This trait provides nothing
//! more than a method for retrieving the ID associated with the given register.
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

pub use byte_array::{FromByteArray, ToByteArray};
pub use regiface_macros::{register, ReadableRegister, WritableRegister};

pub mod byte_array;
pub mod errors;
pub mod i2c;
pub mod register_id;
pub mod spi;

/// The core trait to be implemented for all types that represent readable or writable register values
///
/// This trait provides minimal value on its own, but is a building block to be combined with either [`ReadableRegister`]
/// or [`WritableRegister`].
pub trait Register {
    /// The type used to represent the register's ID.
    ///
    /// Register ID types are any type that implement the [`RegisterId`](register_id::RegisterId) trait. This
    /// trait provides default implementations for [`u8`], [`u16`], [`u32`], [`u64`], and [`u128`].
    type IdType: register_id::RegisterId;

    /// A method that returns the ID of the register for the associated type
    fn id() -> Self::IdType;
}

/// A marker trait that represents a type that can be retrieved by reading a register
///
/// This trait can be manually implemented, or may be derived as such
///
/// ```
/// use regiface::{register, ReadableRegister, FromByteArray};
///
/// #[register(42u8)]
/// #[derive(ReadableRegister, Debug)]
/// pub struct MyRegister {
///     foo: u8
/// }
///
/// impl FromByteArray for MyRegister {
///     type Error = core::convert::Infallible;
///     type Array = [u8; 1];
///
///     fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
///         Ok(Self { foo: bytes[0] })
///     }
/// }
/// ```
pub trait ReadableRegister: Register + FromByteArray {
    /// Some implementations may specify a different register ID to be used when reading the register.
    ///
    /// Override the function if you need to specify an ID value different than that specified by the [`Register`]
    /// implementation for the purpose of writing from the register
    #[inline]
    fn readable_id() -> Self::IdType {
        Self::id()
    }
}

/// A marker trait that represents a type that can be written into a register
///
/// This trait can be manually implemented, or may be derived as such
///
/// ```
/// use regiface::{register, WritableRegister, ToByteArray};
///
/// #[register(42u8)]
/// #[derive(WritableRegister, Debug)]
/// pub struct MyRegister {
///     foo: u8
/// }
///
/// impl ToByteArray for MyRegister {
///     type Error = core::convert::Infallible;
///     type Array = [u8; 1];
///
///     fn to_bytes(self) -> Result<Self::Array, Self::Error> {
///         Ok([self.foo])
///     }
/// }
/// ```
pub trait WritableRegister: Register + ToByteArray {
    /// Some implementations may specify a different register ID to be used when writing the register.
    ///
    /// Override the function if you need to specify an ID value different than that specified by the [`Register`]
    /// implementation for the purpose of writing to the register
    #[inline]
    fn writeable_id() -> Self::IdType {
        Self::id()
    }
}
