use std::convert::Infallible;

pub trait ByteArray: private::Sealed {
    fn new() -> Self;
    fn as_ref(&self) -> &[u8];
    fn as_mut(&mut self) -> &mut [u8];
}

mod private {
    pub trait Sealed {}

    impl<const LEN: usize> Sealed for [u8; LEN] {}
}

impl<const LEN: usize> ByteArray for [u8; LEN] {
    #[inline]
    fn new() -> Self {
        [0; LEN]
    }

    #[inline]
    fn as_ref(&self) -> &[u8] {
        self
    }

    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}

/// A trait to be implemented by any type that can be created from an array of bytes
///
/// If the `packed_struct` feature is used, this trait will be derived for any type that implements derives
/// `PackedStruct` from the [`packed_struct` crate](https://crates.io/crates/packed_struct)
pub trait FromByteArray: Sized {
    /// A type representing the types of error that may occur during conversion
    type Error;
    /// The array of bytes that this value can be converted from
    ///
    /// This value must be a byte array of a specified length, for example `[u8; 5]` or `[u8; 1]`
    type Array: ByteArray;

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error>;
}

impl FromByteArray for u8 {
    type Error = Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}

impl FromByteArray for u16 {
    type Error = Infallible;
    type Array = [u8; 2];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}

impl FromByteArray for u32 {
    type Error = Infallible;
    type Array = [u8; 4];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}

impl FromByteArray for u64 {
    type Error = Infallible;
    type Array = [u8; 8];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}

impl FromByteArray for u128 {
    type Error = Infallible;
    type Array = [u8; 16];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}

#[cfg(packed_struct)]
impl<V, const LEN: usize> FromByteArray<LEN> for V
where
    V: packed_struct::PackedStruct<ByteArray = [u8; LEN]>,
{
    type Error = packed_struct::PackingError;
    type Array = [u8; LEN];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        V::unpack(&bytes)
    }
}

/// A trait to be implemented by any type that can be serialized into an array of bytes
///
/// If the `packed_struct` feature is used, this trait will be derived for any type that implements derives
/// `PackedStruct` from the [`packed_struct` crate](https://crates.io/crates/packed_struct)
pub trait ToByteArray {
    /// A type representing the types of error that may occur during conversion
    type Error;
    /// The array of bytes that this value can be converted into
    ///
    /// This value must be a byte array of a specified length, for example `[u8; 5]` or `[u8; 1]`
    type Array: ByteArray;

    fn to_bytes(self) -> Result<Self::Array, Self::Error>;
}

impl ToByteArray for u8 {
    type Error = Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([self])
    }
}

impl ToByteArray for u16 {
    type Error = Infallible;
    type Array = [u8; 2];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.to_be_bytes())
    }
}

impl ToByteArray for u32 {
    type Error = Infallible;
    type Array = [u8; 4];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.to_be_bytes())
    }
}

impl ToByteArray for u64 {
    type Error = Infallible;
    type Array = [u8; 8];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.to_be_bytes())
    }
}

impl ToByteArray for u128 {
    type Error = Infallible;
    type Array = [u8; 16];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.to_be_bytes())
    }
}

#[cfg(packed_struct)]
impl<V, const LEN: usize> ToByteArray<LEN> for V
where
    V: packed_struct::PackedStruct<ByteArray = [u8; LEN]>,
{
    type Error = packed_struct::PackingError;
    type Array = [u8; LEN];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        self.pack()
    }
}
