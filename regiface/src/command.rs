use crate::{id, FromByteArray, ToByteArray};

/// The core trait to be implemented for all types that represent an invokable command
///
/// All [`Command`]s specify the parameters for both the command and the response body. In the
/// case where either the command or response has no parameters, the [`NoParameters`](crate::NoParameters)
/// can be specified.
///
/// # Example
///
/// ```rust
/// use regiface::{Command, ToByteArray, FromByteArray, NoParameters};
///
/// struct GetTemperature;
///
/// impl Command for GetTemperature {
///     type IdType = u8;
///     type CommandParameters = NoParameters;
///     type ResponseParameters = Temperature;
///
///     fn id() -> Self::IdType {
///         0x42
///     }
///
///     fn invoking_parameters(self) -> Self::CommandParameters {
///         NoParameters::default()
///     }
/// }
///
/// struct Temperature {
///     celsius: f32
/// }
///
/// impl FromByteArray for Temperature {
///     type Error = core::convert::Infallible;
///     type Array = [u8; 4];
///
///     fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
///         let celsius = f32::from_be_bytes(bytes);
///         Ok(Self { celsius })
///     }
/// }
/// ```
pub trait Command {
    /// The type used to represent the command's ID.
    ///
    /// Command ID types are any type that implement the [`Id`](id::Id) trait. This
    /// trait provides default implementations for [`u8`], [`u16`], [`u32`], [`u64`], and [`u128`].
    type IdType: id::Id;

    /// The parameters included as part of the command invocation
    ///
    /// If the command has no parameters, the [`NoParameters`](crate::NoParameters) type can be used
    type CommandParameters: ToByteArray;

    /// The parameters expected as the response to the command
    ///
    /// If the response has no parameters, the [`NoParameters`](crate::NoParameters) type can be used
    type ResponseParameters: FromByteArray;

    /// A method that returns the ID of the [`Command`]
    fn id() -> Self::IdType;

    /// A method to retrieve the parameters from an instance of the [`Command`]
    fn invoking_parameters(self) -> Self::CommandParameters;
}

/// A utility type for use when defining a [`Command`] that should pass no parameters, or
/// a [`Command`] that returns no parameters.
///
/// Instances of [`NoParameters`] should be constructed using the `default()` implementation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Default)]
pub struct NoParameters {}

/// A utility type for use when defining a [`Command`] that should pass a set of zero
/// values as its command parameters.
///
/// Instances of [`Zeros`] should be constructed using the `default()` implementation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Default)]
pub struct Zeros<const N: usize = 0> {}
