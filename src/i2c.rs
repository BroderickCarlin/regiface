use crate::{
    ByteArray as _, FromByteArray, ReadRegisterError, ReadableRegister, WritableRegister,
    WriteRegisterError,
};

pub trait ReadableI2cRegister: ReadableRegister {
    #[inline]
    async fn read_async<D, A>(
        device: &mut D,
        device_addr: A,
    ) -> Result<Self, ReadRegisterError<D::Error, Self::Error>>
    where
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
    {
        let mut buf = <Self as FromByteArray>::Array::new();

        device
            .write_read(device_addr, &[Self::readable_id()], buf.as_mut())
            .await
            .map_err(ReadRegisterError::BusError)?;

        Self::from_array(buf).map_err(ReadRegisterError::DeserializationError)
    }

    #[inline]
    fn read<D, A>(
        device: &mut D,
        device_addr: A,
    ) -> Result<Self, ReadRegisterError<D::Error, Self::Error>>
    where
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
    {
        let mut buf = <Self as FromByteArray>::Array::new();

        device
            .write_read(device_addr, &[Self::readable_id()], buf.as_mut())
            .map_err(ReadRegisterError::BusError)?;

        Self::from_array(buf).map_err(ReadRegisterError::DeserializationError)
    }
}

/// A trait for registers that are writable on an I2C device
pub trait WritableI2cRegister<const LEN: usize>: WritableRegister {
    #[inline]
    async fn write_async<D, A>(
        self,
        device: &mut D,
        device_addr: A,
    ) -> Result<(), WriteRegisterError<D::Error, Self::Error>>
    where
        Self: Sized,
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
    {
        let buf = self
            .to_array()
            .map_err(WriteRegisterError::SerializationError)?;

        device
            .transaction(
                device_addr,
                &mut [
                    embedded_hal_async::i2c::Operation::Write(&[Self::writeable_id()]),
                    embedded_hal_async::i2c::Operation::Write(buf.as_ref()),
                ],
            )
            .await
            .map_err(WriteRegisterError::BusError)
    }

    #[inline]
    fn write<D, A>(
        self,
        device: &mut D,
        device_addr: A,
    ) -> Result<(), WriteRegisterError<D::Error, Self::Error>>
    where
        Self: Sized,
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
    {
        let buf = self
            .to_array()
            .map_err(WriteRegisterError::SerializationError)?;

        device
            .transaction(
                device_addr,
                &mut [
                    embedded_hal::i2c::Operation::Write(&[Self::writeable_id()]),
                    embedded_hal::i2c::Operation::Write(buf.as_ref()),
                ],
            )
            .map_err(WriteRegisterError::BusError)
    }
}
