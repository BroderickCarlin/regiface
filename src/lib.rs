use ::embedded_hal::i2c::{AddressMode, I2c, Operation};
pub use slice::*;
use thiserror::Error;

mod slice;

#[derive(Clone, Copy, Debug, Error)]
pub enum ReadRegisterError<D, S> {
    BusError(D),
    DeserializationError(S),
}

#[derive(Clone, Copy, Debug, Error)]
pub enum WriteRegisterError<D, S> {
    BusError(D),
    SerializationError(S),
}

/// The generic top level trait for all register values
pub trait Register {
    fn id() -> u8;
}

pub trait ReadableI2cRegister<const N: usize = 1>: Register + FromSlice<N> {
    fn readable_id() -> u8 {
        Self::id() | 0x40
    }

    fn read<D, A>(
        device: &mut D,
        device_addr: A,
    ) -> Result<Self, ReadRegisterError<D::Error, Self::Error>>
    where
        A: AddressMode,
        D: I2c<A>,
    {
        let mut buf = [0u8; N];

        device
            .write_read(device_addr, &[Self::readable_id()], &mut buf)
            .map_err(ReadRegisterError::BusError)?;

        Self::from_slice(buf).map_err(ReadRegisterError::DeserializationError)
    }
}

/// A trait for registers that are writable on an I2C device
pub trait WritableI2cRegister<const N: usize = 1>: Register + IntoSlice<N> {
    fn writeable_id() -> u8 {
        Self::id()
    }

    fn write<D, A>(
        self,
        device: &mut D,
        device_addr: A,
    ) -> Result<(), WriteRegisterError<D::Error, Self::Error>>
    where
        Self: Sized,
        A: AddressMode,
        D: I2c<A>,
    {
        let buf = self
            .into_slice()
            .map_err(WriteRegisterError::SerializationError)?;

        device
            .transaction(
                device_addr,
                &mut [
                    Operation::Write(&[Self::writeable_id()]),
                    Operation::Write(&buf),
                ],
            )
            .map_err(WriteRegisterError::BusError)
    }
}
