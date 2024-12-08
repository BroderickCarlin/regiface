//! A collection of utility functions for interfacing with registers across an I2C bus
//!
//! Provided are both blocking and async variants of all functions

use crate::{
    byte_array::ByteArray as _,
    errors::{ReadRegisterError, ReadRegisterResult, WriteRegisterError, WriteRegisterResult},
    FromByteArray, ReadableRegister, ToByteArray as _, WritableRegister,
};

pub mod r#async {
    use super::*;

    /// A utility method to reading registers from an I2C device, asynchronously
    pub async fn read_register<D, A, R>(
        device: &mut D,
        device_addr: A,
    ) -> ReadRegisterResult<D::Error, R>
    where
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { R::readable_id().to_bytes().unwrap_unchecked() };

        device
            .write_read(device_addr, reg_id.as_ref(), buf.as_mut())
            .await
            .map_err(ReadRegisterError::BusError)?;

        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }

    /// A utility method to write registers to an I2C device, asynchronously
    pub async fn write_register<D, A, R>(
        device: &mut D,
        device_addr: A,
        register: R,
    ) -> WriteRegisterResult<D::Error, R>
    where
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { R::writeable_id().to_bytes().unwrap_unchecked() };

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

    /// A utility method to reading registers from an I2C device, in a blocking manner
    pub fn read_register<D, A, R>(device: &mut D, device_addr: A) -> ReadRegisterResult<D::Error, R>
    where
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { R::readable_id().to_bytes().unwrap_unchecked() };

        device
            .write_read(device_addr, reg_id.as_ref(), buf.as_mut())
            .map_err(ReadRegisterError::BusError)?;

        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }

    /// A utility method to write registers to an I2C device, in a blocking manner
    pub fn write_register<D, A, R>(
        device: &mut D,
        device_addr: A,
        register: R,
    ) -> WriteRegisterResult<D::Error, R>
    where
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { R::writeable_id().to_bytes().unwrap_unchecked() };

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
