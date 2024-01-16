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

pub trait FromByteArray: Sized {
    type Error;
    type Array: ByteArray;

    fn from_array(bytes: Self::Array) -> Result<Self, Self::Error>;
}

#[cfg(packed_struct)]
impl<V, const LEN: usize> FromByteArray<LEN> for V
where
    V: packed_struct::PackedStruct<ByteArray = [u8; LEN]>,
{
    type Error = packed_struct::PackingError;
    type Array = [u8; LEN];

    fn from_array(bytes: Self::Array) -> Result<Self, Self::Error> {
        V::unpack(&bytes)
    }
}

pub trait ToByteArray {
    type Error;
    type Array: ByteArray;

    fn to_array(self) -> Result<Self::Array, Self::Error>;
}

#[cfg(packed_struct)]
impl<V, const LEN: usize> ToByteArray<LEN> for V
where
    V: packed_struct::PackedStruct<ByteArray = [u8; LEN]>,
{
    type Error = packed_struct::PackingError;
    type Array = [u8; LEN];

    fn to_array(self) -> Result<Self::Array, Self::Error> {
        self.pack()
    }
}
