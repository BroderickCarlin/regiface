use ::embedded_hal::i2c::{AddressMode, I2c, Operation};
use packed_struct::{types::bits::ByteArray as _, PackedStruct, PackingError};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum ReadRegisterError<D> {
    BusError(D),
    DeserializationError(PackingError),
}

#[derive(Clone, Copy, Debug, Error)]
pub enum WriteRegisterError<D> {
    BusError(D),
    SerializationError(PackingError),
}

/// The generic top level trait for all register values
pub trait Register {
    fn id() -> u8;
}

pub trait ReadableI2cRegister: Register + PackedStruct {
    #[inline]
    fn readable_id() -> u8 {
        Self::id() | 0x40
    }

    #[inline]
    fn read<D, A>(device: &mut D, device_addr: A) -> Result<Self, ReadRegisterError<D::Error>>
    where
        A: AddressMode,
        D: I2c<A>,
    {
        let mut buf = Self::ByteArray::new(0);

        device
            .write_read(
                device_addr,
                &[Self::readable_id()],
                buf.as_mut_bytes_slice(),
            )
            .map_err(ReadRegisterError::BusError)?;

        Self::unpack(&buf).map_err(ReadRegisterError::DeserializationError)
    }
}

/// A trait for registers that are writable on an I2C device
pub trait WritableI2cRegister: Register + PackedStruct {
    #[inline]
    fn writeable_id() -> u8 {
        Self::id()
    }

    #[inline]
    fn write<D, A>(self, device: &mut D, device_addr: A) -> Result<(), WriteRegisterError<D::Error>>
    where
        Self: Sized,
        A: AddressMode,
        D: I2c<A>,
    {
        let buf = self
            .pack()
            .map_err(WriteRegisterError::SerializationError)?;

        device
            .transaction(
                device_addr,
                &mut [
                    Operation::Write(&[Self::writeable_id()]),
                    Operation::Write(buf.as_bytes_slice()),
                ],
            )
            .map_err(WriteRegisterError::BusError)
    }
}
