//! A collection of utility functions for interfacing with registers across a SPI bus
//!
//! Provided are both blocking and async variants of all functions

use crate::{
    byte_array::ByteArray as _,
    errors::{ReadRegisterError, WriteRegisterError},
    FromByteArray, ReadableRegister, ToByteArray as _, WritableRegister,
};

pub mod r#async {
    use super::*;

    /// Read the specified register value from the provided [`SpiDevice`](embedded_hal_async::spi::SpiDevice)
    pub async fn read_register<D, R>(
        device: &mut D,
    ) -> Result<R, ReadRegisterError<D::Error, R::Error>>
    where
        D: embedded_hal_async::spi::SpiDevice,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::readable_id().to_bytes().unwrap();

        device
            .transaction(&mut [
                embedded_hal_async::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal_async::spi::Operation::Read(buf.as_mut()),
            ])
            .await
            .map_err(ReadRegisterError::BusError)?;

        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }

    /// Write the specified register value to the provided [`SpiDevice`](embedded_hal_async::spi::SpiDevice)
    pub async fn write_register<D, R>(
        device: &mut D,
        register: R,
    ) -> Result<(), WriteRegisterError<D::Error, R::Error>>
    where
        D: embedded_hal_async::spi::SpiDevice,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::writeable_id().to_bytes().unwrap();

        device
            .transaction(&mut [
                embedded_hal_async::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal_async::spi::Operation::Write(buf.as_ref()),
            ])
            .await
            .map_err(WriteRegisterError::BusError)
    }
}

pub mod blocking {
    use super::*;

    /// Read the specified register value from the provided [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub fn read_register<D, R>(device: &mut D) -> Result<R, ReadRegisterError<D::Error, R::Error>>
    where
        D: embedded_hal::spi::SpiDevice,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::readable_id().to_bytes().unwrap();

        device
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal::spi::Operation::Read(buf.as_mut()),
            ])
            .map_err(ReadRegisterError::BusError)?;
 
        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }

    /// Write the specified register value to the provided [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub fn write_register<D, R>(
        device: &mut D,
        register: R,
    ) -> Result<(), WriteRegisterError<D::Error, R::Error>>
    where
        D: embedded_hal::spi::SpiDevice,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::writeable_id().to_bytes().unwrap();

        device
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal::spi::Operation::Write(buf.as_ref()),
            ])
            .map_err(WriteRegisterError::BusError)
    }
}
