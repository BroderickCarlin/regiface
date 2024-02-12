//! A collection of utility functions for interfacing with registers across an I2C bus
//!
//! Provided are both blocking and async variants of all functions

use crate::{
    byte_array::ByteArray as _,
    errors::{ReadRegisterError, WriteRegisterError},
    FromByteArray, ReadableRegister, ToByteArray as _, WritableRegister,
};

pub mod r#async {
    use super::*;

    pub async fn read_register<D, A, R>(
        device: &mut D,
        device_addr: A,
    ) -> Result<R, ReadRegisterError<D::Error, R::Error>>
    where
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::readable_id().to_bytes().unwrap();

        device
            .write_read(device_addr, reg_id.as_ref(), buf.as_mut())
            .await
            .map_err(ReadRegisterError::BusError)?;

        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }

    pub async fn write_register<D, A, R>(
        device: &mut D,
        device_addr: A,
        register: R,
    ) -> Result<(), WriteRegisterError<D::Error, R::Error>>
    where
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::writeable_id().to_bytes().unwrap();

        device
            .transaction(
                device_addr,
                &mut [
                    embedded_hal_async::i2c::Operation::Write(reg_id.as_ref()),
                    embedded_hal_async::i2c::Operation::Write(buf.as_ref()),
                ],
            )
            .await
            .map_err(WriteRegisterError::BusError)
    }
}

pub mod blocking {
    use super::*;

    pub fn read_register<D, A, R>(
        device: &mut D,
        device_addr: A,
    ) -> Result<R, ReadRegisterError<D::Error, R::Error>>
    where
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::readable_id().to_bytes().unwrap();

        device
            .write_read(device_addr, reg_id.as_ref(), buf.as_mut())
            .map_err(ReadRegisterError::BusError)?;

        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }

    pub fn write_register<D, A, R>(
        device: &mut D,
        device_addr: A,
        register: R,
    ) -> Result<(), WriteRegisterError<D::Error, R::Error>>
    where
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = R::writeable_id().to_bytes().unwrap();

        device
            .transaction(
                device_addr,
                &mut [
                    embedded_hal_async::i2c::Operation::Write(reg_id.as_ref()),
                    embedded_hal_async::i2c::Operation::Write(buf.as_ref()),
                ],
            )
            .map_err(WriteRegisterError::BusError)
    }
}
