//! A collection of utility functions for interfacing with registers across a SPI bus
//!
//! Provided are both blocking and async variants of all functions

use crate::{
    byte_array::ByteArray as _,
    errors::CommandError,
    errors::{ReadRegisterError, WriteRegisterError},
    Command, FromByteArray, ReadableRegister, ToByteArray, WritableRegister,
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

    #[allow(clippy::type_complexity)]
    pub async fn invoke_command<D, C>(
        device: &mut D,
        cmd: C,
    ) -> Result<
        C::ResponseParameters,
        CommandError<
            D::Error,
            <C::CommandParameters as ToByteArray>::Error,
            <C::ResponseParameters as FromByteArray>::Error,
        >,
    >
    where
        D: embedded_hal_async::spi::SpiDevice,
        C: Command,
    {
        let cmd_buf = cmd
            .invoking_parameters()
            .to_bytes()
            .map_err(CommandError::SerializationError)?;
        let mut resp_buf = <C::ResponseParameters as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { C::id().to_bytes().unwrap_unchecked() };

        device
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal::spi::Operation::Write(cmd_buf.as_ref()),
                embedded_hal::spi::Operation::Read(resp_buf.as_mut()),
            ])
            .await
            .map_err(CommandError::BusError)?;

        C::ResponseParameters::from_bytes(resp_buf).map_err(CommandError::DeserializationError)
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
        let reg_id = unsafe { R::readable_id().to_bytes().unwrap_unchecked() };

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
        let reg_id = unsafe { R::writeable_id().to_bytes().unwrap_unchecked() };

        device
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal::spi::Operation::Write(buf.as_ref()),
            ])
            .map_err(WriteRegisterError::BusError)
    }

    #[allow(clippy::type_complexity)]
    pub fn invoke_command<D, C>(
        device: &mut D,
        cmd: C,
    ) -> Result<
        C::ResponseParameters,
        CommandError<
            D::Error,
            <C::CommandParameters as ToByteArray>::Error,
            <C::ResponseParameters as FromByteArray>::Error,
        >,
    >
    where
        D: embedded_hal::spi::SpiDevice,
        C: Command,
    {
        let cmd_buf = cmd
            .invoking_parameters()
            .to_bytes()
            .map_err(CommandError::SerializationError)?;
        let mut resp_buf = <C::ResponseParameters as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { C::id().to_bytes().unwrap_unchecked() };

        device
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal::spi::Operation::Write(cmd_buf.as_ref()),
                embedded_hal::spi::Operation::Read(resp_buf.as_mut()),
            ])
            .map_err(CommandError::BusError)?;

        C::ResponseParameters::from_bytes(resp_buf).map_err(CommandError::DeserializationError)
    }
}
