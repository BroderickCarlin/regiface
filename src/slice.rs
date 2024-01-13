use std::{convert::Infallible, error::Error};

/// A utility trait for representing types that can be created from a slice of bytes of a specific length
///
/// It is suggested to instead implement `From<u8>`, `From<u16>`, or `From<u32>` as this trait will be auto-derived
/// for types that do.
pub trait FromSlice<const N: usize>: Sized {
    type Error: Error;

    fn from_slice(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

impl<T> FromSlice<1> for T
where
    T: From<u8>,
{
    type Error = Infallible;

    fn from_slice(bytes: [u8; 1]) -> Result<Self, Self::Error> {
        Ok(Self::from(bytes[0]))
    }
}

impl<T> FromSlice<2> for T
where
    T: From<u16>,
{
    type Error = Infallible;

    fn from_slice(bytes: [u8; 2]) -> Result<Self, Self::Error> {
        Ok(Self::from(u16::from_le_bytes(bytes)))
    }
}

impl<T> FromSlice<4> for T
where
    T: From<u32>,
{
    type Error = Infallible;

    fn from_slice(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self::from(u32::from_le_bytes(bytes)))
    }
}

/// A utility trait for representing types that can be converted into a slice of bytes of a specified length
///
/// It is suggested to instead implement `Into<u8>`, `Into<u16>`, or `Into<u32>` as this trait will be auto-derived
/// for types that do.
pub trait IntoSlice<const N: usize> {
    type Error: Error;

    fn into_slice(self) -> Result<[u8; N], Self::Error>;
}

impl<T> IntoSlice<1> for T
where
    T: Into<u8>,
{
    type Error = Infallible;

    fn into_slice(self) -> Result<[u8; 1], Self::Error> {
        Ok([self.into()])
    }
}

impl<T> IntoSlice<2> for T
where
    T: Into<u16>,
{
    type Error = Infallible;

    fn into_slice(self) -> Result<[u8; 2], Self::Error> {
        Ok(self.into().to_le_bytes())
    }
}

impl<T> IntoSlice<4> for T
where
    T: Into<u32>,
{
    type Error = Infallible;

    fn into_slice(self) -> Result<[u8; 4], Self::Error> {
        Ok(self.into().to_le_bytes())
    }
}
