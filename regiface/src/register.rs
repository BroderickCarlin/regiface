use crate::{id, FromByteArray, ToByteArray};

/// The core trait to be implemented for all types that represent readable or writable register values
///
/// This trait provides minimal value on its own, but is a building block to be combined with either [`ReadableRegister`]
/// or [`WritableRegister`].
pub trait Register {
    /// The type used to represent the register's ID.
    ///
    /// Register ID types are any type that implement the [`Id`](id::Id) trait. This
    /// trait provides default implementations for [`u8`], [`u16`], [`u32`], [`u64`], and [`u128`].
    type IdType: id::Id;

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
