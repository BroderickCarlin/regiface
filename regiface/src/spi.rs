//! A collection of utility functions for interfacing with registers across a SPI bus
//!
//! Provided are both blocking and async variants of all functions

use std::future::Future;
use std::pin::Pin;

use crate::{
    byte_array::ByteArray as _,
    errors::{ReadRegisterError, ReadRegisterResult, WriteRegisterError, WriteRegisterResult},
    FromByteArray, ReadableRegister, Register, ToByteArray, WritableRegister,
};

pub fn read_register<'d, D, R>(device: &'d mut D) -> SpiRegisterReader<'d, D, R>
where
    R: ReadableRegister,
{
    SpiRegisterReader::new(device)
}

pub struct SpiRegisterReader<'d, D, R>
where
    R: ReadableRegister,
{
    device: &'d mut D,
    buf: <R as FromByteArray>::Array,
    reg_id: <<R as Register>::IdType as ToByteArray>::Array,
    _type: std::marker::PhantomData<R>,
}

impl<'d, D, R> SpiRegisterReader<'d, D, R>
where
    R: ReadableRegister,
{
    fn new(device: &'d mut D) -> Self {
        Self {
            device,
            buf: <R as FromByteArray>::Array::new(),
            // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
            reg_id: unsafe { R::readable_id().to_bytes().unwrap_unchecked() },
            _type: Default::default(),
        }
    }
}

impl<'d, D, R> SpiRegisterReader<'d, D, R>
where
    R: ReadableRegister,
    D: embedded_hal::spi::SpiDevice,
{
    fn block(self) -> ReadRegisterResult<D::Error, R> {
        let Self {
            device,
            mut buf,
            reg_id,
            ..
        } = self;

        device
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(reg_id.as_ref()),
                embedded_hal::spi::Operation::Read(buf.as_mut()),
            ])
            .map_err(ReadRegisterError::BusError)?;

        R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }
}

impl<'d, D, R> Future for SpiRegisterReader<'d, D, R>
where
    D: embedded_hal_async::spi::SpiDevice,
    R: ReadableRegister,
{
    type Output = ReadRegisterResult<D::Error, R>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        unimplemented!()

        // device
        //     .transaction(&mut [
        //         embedded_hal_async::spi::Operation::Write(reg_id.as_ref()),
        //         embedded_hal_async::spi::Operation::Read(buf.as_mut()),
        //     ])
        //     .await
        //     .map_err(ReadRegisterError::BusError)?;

        // R::from_bytes(buf).map_err(ReadRegisterError::DeserializationError)
    }
}

pub mod r#async {
    use super::*;

    /// Read the specified register value from the provided [`SpiDevice`](embedded_hal_async::spi::SpiDevice)
    pub async fn read_register<D, R>(device: &mut D) -> ReadRegisterResult<D::Error, R>
    where
        D: embedded_hal_async::spi::SpiDevice,
        R: ReadableRegister,
    {
        let mut buf = <R as FromByteArray>::Array::new();

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { R::readable_id().to_bytes().unwrap_unchecked() };

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
    ) -> WriteRegisterResult<D::Error, R>
    where
        D: embedded_hal_async::spi::SpiDevice,
        R: WritableRegister,
    {
        let buf = register
            .to_bytes()
            .map_err(WriteRegisterError::SerializationError)?;

        // Register ID types have compiler enforced infallible byte conversions, thus this unwrap is safe
        let reg_id = unsafe { R::writeable_id().to_bytes().unwrap_unchecked() };

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

    /// Write the specified register value to the provided [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub fn write_register<D, R>(device: &mut D, register: R) -> WriteRegisterResult<D::Error, R>
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
}
