//! A collection of utility functions for interfacing with registers across an I2C bus
//!
//! This module provides both blocking and async variants of register read/write operations
//! and command invocation for I2C devices. All operations handle device addressing and
//! proper byte serialization/deserialization of register values.

use crate::{
    byte_array::ByteArray as _,
    errors::CommandError,
    errors::{ReadRegisterError, WriteRegisterError},
    Command, FromByteArray, ReadableRegister, ToByteArray, WritableRegister,
};

pub mod r#async {
    use super::*;

    /// Read a register value from an I2C device.
    ///
    /// This function performs a write-read I2C transaction, first sending the register ID
    /// then reading the register value. The received bytes are deserialized into the
    /// specified register type.
    ///
    /// # Parameters
    /// * `device` - The I2C device to communicate with
    /// * `device_addr` - The I2C address of the target device
    ///
    /// # Errors
    /// * `ReadRegisterError::BusError` - Communication with the device failed
    /// * `ReadRegisterError::DeserializationError` - Failed to convert received bytes into register value
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal_async::i2c::I2c;
    /// # use regiface::{register, i2c, ReadableRegister, FromByteArray};
    /// # #[register(42u8)]
    /// # #[derive(ReadableRegister)]
    /// # struct TemperatureRegister;
    /// # impl FromByteArray for TemperatureRegister {
    /// #     type Array = [u8; 2];
    /// #     type Error = ();
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> { todo!() }
    /// # }
    /// async fn read_temp<D: I2c<u8>>(device: &mut D) {
    ///     let temp: TemperatureRegister = i2c::r#async::read_register(device, 0x48).await.unwrap();
    /// }
    /// ```
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

    /// Write a register value to an I2C device.
    ///
    /// This function performs a write I2C transaction, sending both the register ID
    /// and the serialized register value. The operation is atomic, using the device's
    /// transaction capability to ensure both writes occur without interruption.
    ///
    /// # Parameters
    /// * `device` - The I2C device to communicate with
    /// * `device_addr` - The I2C address of the target device
    /// * `register` - The register value to write
    ///
    /// # Errors
    /// * `WriteRegisterError::BusError` - Communication with the device failed
    /// * `WriteRegisterError::SerializationError` - Failed to convert register value to bytes
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal_async::i2c::I2c;
    /// # use regiface::{register, i2c, WritableRegister, ToByteArray};
    /// #[register(42u8)]
    /// #[derive(WritableRegister)]
    /// # struct ConfigRegister{};
    /// # impl ToByteArray for ConfigRegister {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn to_bytes(self) -> Result<Self::Array, Self::Error> { Ok([0]) }
    /// # }
    /// async fn configure<D: I2c<u8>>(device: &mut D) {
    ///     i2c::r#async::write_register(device, 0x48, ConfigRegister{ /* ... */}).await.unwrap();
    /// }
    /// ```
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

    /// Invoke a command on an I2C device and receive its response.
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
    /// * `device` - The I2C device to communicate with
    /// * `device_addr` - The I2C address of the target device
    /// * `cmd` - The command to invoke
    ///
    /// # Errors
    /// * `CommandError::BusError` - Communication with the device failed
    /// * `CommandError::SerializationError` - Failed to convert command parameters to bytes
    /// * `CommandError::DeserializationError` - Failed to convert received bytes into response parameters
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal_async::i2c::I2c;
    /// # use regiface::{NoParameters, i2c, Command, FromByteArray, ToByteArray};
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
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> { Ok(Self) }
    /// # }
    /// async fn perform_self_test<D: I2c<u8>>(device: &mut D) {
    ///     let result: SelfTestResponse = i2c::r#async::invoke_command(device, 0x48, SelfTestCommand{ /* ... */}).await.unwrap();
    /// }
    /// ```
    #[allow(clippy::type_complexity)]
    pub async fn invoke_command<D, A, C>(
        device: &mut D,
        device_addr: A,
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
        A: embedded_hal_async::i2c::AddressMode,
        D: embedded_hal_async::i2c::I2c<A>,
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
            .transaction(
                device_addr,
                &mut [
                    embedded_hal::i2c::Operation::Write(reg_id.as_ref()),
                    embedded_hal::i2c::Operation::Write(cmd_buf.as_ref()),
                    embedded_hal::i2c::Operation::Read(resp_buf.as_mut()),
                ],
            )
            .await
            .map_err(CommandError::BusError)?;

        C::ResponseParameters::from_bytes(resp_buf).map_err(CommandError::DeserializationError)
    }
}

pub mod blocking {
    use super::*;

    /// Read a register value from an I2C device.
    ///
    /// Blocking variant of [`read_register`](crate::i2c::async::read_register).
    /// See the async function documentation for detailed behavior description.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal::i2c::I2c;
    /// # use regiface::{register, i2c, ReadableRegister, FromByteArray};
    /// # #[register(42u8)]
    /// # #[derive(ReadableRegister)]
    /// # struct TemperatureRegister;
    /// # impl FromByteArray for TemperatureRegister {
    /// #     type Array = [u8; 2];
    /// #     type Error = ();
    /// #     fn from_bytes(_: Self::Array) -> Result<Self, Self::Error> {todo!()}
    /// # }
    /// fn read_temp<D: I2c<u8>>(device: &mut D) {
    ///     let temp: TemperatureRegister = i2c::blocking::read_register(device, 0x48).unwrap();
    /// }
    /// ```
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

    /// Write a register value to an I2C device.
    ///
    /// Blocking variant of [`write_register`](crate::i2c::async::write_register).
    /// See the async function documentation for detailed behavior description.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal::i2c::I2c;
    /// # use regiface::{register, i2c, WritableRegister, ToByteArray};
    /// #[register(42u8)]
    /// #[derive(WritableRegister)]
    /// # struct ConfigRegister;
    /// # impl ToByteArray for ConfigRegister {
    /// #     type Array = [u8; 1];
    /// #     type Error = ();
    /// #     fn to_bytes(self) -> Result<Self::Array, Self::Error> { Ok([0]) }
    /// # }
    /// fn configure<D: I2c<u8>>(device: &mut D) {
    ///     i2c::blocking::write_register(device, 0x48, ConfigRegister{ /* ... */}).unwrap();
    /// }
    /// ```
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

    /// Invoke a command on an I2C device and receive its response.
    ///
    /// Blocking variant of [`invoke_command`](crate::i2c::async::invoke_command).
    /// See the async function documentation for detailed behavior description.
    ///
    /// # Example
    /// ```no_run
    /// # use embedded_hal::i2c::I2c;
    /// # use regiface::{NoParameters, i2c, Command, FromByteArray, ToByteArray};
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
    /// fn perform_self_test<D: I2c<u8>>(device: &mut D) {
    ///     let result: SelfTestResponse = i2c::blocking::invoke_command(device, 0x48, SelfTestCommand).unwrap();
    /// }
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn invoke_command<D, A, C>(
        device: &mut D,
        device_addr: A,
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
        A: embedded_hal::i2c::AddressMode,
        D: embedded_hal::i2c::I2c<A>,
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
            .transaction(
                device_addr,
                &mut [
                    embedded_hal::i2c::Operation::Write(reg_id.as_ref()),
                    embedded_hal::i2c::Operation::Write(cmd_buf.as_ref()),
                    embedded_hal::i2c::Operation::Read(resp_buf.as_mut()),
                ],
            )
            .map_err(CommandError::BusError)?;

        C::ResponseParameters::from_bytes(resp_buf).map_err(CommandError::DeserializationError)
    }
}
