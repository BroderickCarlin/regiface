//! A collection of utility functions for interfacing with registers across a SPI bus
//!
//! This module provides both blocking and async variants of register read/write operations
//! and command invocation for SPI devices. All operations handle proper byte
//! serialization/deserialization of register values.

use crate::{
    byte_array::ByteArray as _,
    errors::CommandError,
    errors::{ReadRegisterError, WriteRegisterError},
    Command, FromByteArray, ReadableRegister, ToByteArray, WritableRegister,
};

pub mod r#async {
    use super::*;

    /// Read a register value from a SPI device.
    ///
    /// This function performs a SPI transaction, first sending the register ID
    /// then reading the register value. The received bytes are deserialized into the
    /// specified register type.
    ///
    /// # Parameters
    /// * `device` - The SPI device to communicate with
    ///
    /// # Errors
    /// * `ReadRegisterError::BusError` - Communication with the device failed
    /// * `ReadRegisterError::DeserializationError` - Failed to convert received bytes into register value
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal_async::spi::SpiDevice;
    /// # use regiface::{register, spi, ReadableRegister, FromByteArray};
    /// # #[register(1u8)]
    /// # #[derive(ReadableRegister)]
    /// # struct TemperatureRegister{};
    /// # impl FromByteArray for TemperatureRegister {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> { todo!() }
    /// # }
    /// async fn read_temp<D: SpiDevice>(device: &mut D) {
    ///     let temp: TemperatureRegister = spi::r#async::read_register(device).await.unwrap();
    /// }
    /// ```
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

    /// Write a register value to a SPI device.
    ///
    /// This function performs a SPI transaction, sending both the register ID
    /// and the serialized register value. The operation is atomic, using the device's
    /// transaction capability to ensure both writes occur without interruption.
    ///
    /// # Parameters
    /// * `device` - The SPI device to communicate with
    /// * `register` - The register value to write
    ///
    /// # Errors
    /// * `WriteRegisterError::BusError` - Communication with the device failed
    /// * `WriteRegisterError::SerializationError` - Failed to convert register value to bytes
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal_async::spi::SpiDevice;
    /// # use regiface::{register, spi, WritableRegister, ToByteArray};
    /// # #[register(1u8)]
    /// # #[derive(WritableRegister)]
    /// # struct ConfigRegister{};
    /// # impl ToByteArray for ConfigRegister {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn to_bytes(self) -> Result<Self::Array, Self::Error> { Ok([0]) }
    /// # }
    /// async fn configure<D: SpiDevice>(device: &mut D) {
    ///     spi::r#async::write_register(device, ConfigRegister{/* ... */}).await.unwrap();
    /// }
    /// ```
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

    /// Invoke a command on a SPI device and receive its response.
    ///
    /// This function performs a complete command transaction:
    /// 1. Sends the command ID
    /// 2. Sends the serialized command parameters
    /// 3. Reads the command response
    ///
    /// The entire operation is atomic, using the device's transaction capability to
    /// ensure all steps occur without interruption.
    ///
    /// # Parameters
    /// * `device` - The SPI device to communicate with
    /// * `cmd` - The command to invoke
    ///
    /// # Errors
    /// * `CommandError::BusError` - Communication with the device failed
    /// * `CommandError::SerializationError` - Failed to convert command parameters to bytes
    /// * `CommandError::DeserializationError` - Failed to convert received bytes into response parameters
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal_async::spi::SpiDevice;
    /// # use regiface::{NoParameters, spi, Command, FromByteArray, ToByteArray};
    /// # struct SelfTestCommand;
    /// # struct SelfTestResponse;
    /// # impl Command for SelfTestCommand {
    /// #     type IdType = u8;
    /// #     type CommandParameters = NoParameters;
    /// #     type ResponseParameters = SelfTestResponse;
    /// #     fn id() -> Self::IdType { 0xF0 }
    /// #     fn invoking_parameters(self) -> Self::CommandParameters { todo!() }
    /// # }
    /// # impl FromByteArray for SelfTestResponse {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> { Ok(Self) }
    /// # }
    /// async fn perform_self_test<D: SpiDevice>(device: &mut D) {
    ///     let result: SelfTestResponse = spi::r#async::invoke_command(device, SelfTestCommand).await.unwrap();
    /// }
    /// ```
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

    /// Read a register value from a SPI device.
    ///
    /// Blocking variant of [`read_register`](crate::spi::async::read_register).
    /// See the async function documentation for detailed behavior description.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal::spi::SpiDevice;
    /// # use regiface::{register, spi, ReadableRegister, FromByteArray};
    /// # #[register(42u8)]
    /// # #[derive(ReadableRegister)]
    /// # struct TemperatureRegister;
    /// # impl FromByteArray for TemperatureRegister {
    /// #     type Array = [u8; 2];
    /// #     type Error = ();
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> {todo!()}
    /// # }
    /// fn read_temp<D: SpiDevice>(device: &mut D) {
    ///     let temp: TemperatureRegister = spi::blocking::read_register(device).unwrap();
    /// }
    /// ```
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

    /// Write a register value to a SPI device.
    ///
    /// Blocking variant of [`write_register`](crate::spi::async::write_register).
    /// See the async function documentation for detailed behavior description.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal::spi::SpiDevice;
    /// # use regiface::{register, spi, WritableRegister, ToByteArray};
    /// # #[register(42u8)]
    /// # #[derive(WritableRegister)]
    /// # struct ConfigRegister{};
    /// # impl ToByteArray for ConfigRegister {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn to_bytes(self) -> Result<Self::Array, Self::Error> { Ok([0]) }
    /// # }
    /// fn configure<D: SpiDevice>(device: &mut D) {
    ///     spi::blocking::write_register(device, ConfigRegister{ /* ... */}).unwrap();
    /// }
    /// ```
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

    /// Invoke a command on a SPI device and receive its response.
    ///
    /// Blocking variant of [`invoke_command`](crate::spi::async::invoke_command).
    /// See the async function documentation for detailed behavior description.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal::spi::SpiDevice;
    /// # use regiface::{NoParameters, spi, Command, FromByteArray, ToByteArray};
    /// # struct SelfTestCommand{};
    /// # struct SelfTestResponse;
    /// # impl Command for SelfTestCommand {
    /// #     type IdType = u8;
    /// #     type CommandParameters = NoParameters;
    /// #     type ResponseParameters = SelfTestResponse;
    /// #     fn id() -> Self::IdType { 0xF0 }
    /// #     fn invoking_parameters(self) -> Self::CommandParameters { todo!() }
    /// # }
    /// # impl FromByteArray for SelfTestResponse {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> { todo!() }
    /// # }
    /// fn perform_self_test<D: SpiDevice>(device: &mut D) {
    ///     let result: SelfTestResponse = spi::blocking::invoke_command(device, SelfTestCommand { /* ... */}).unwrap();
    /// }
    /// ```
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
