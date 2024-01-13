use embedded_hal::i2c::{AddressMode, I2c, Operation};
use thiserror::Error;

use crate::{ReadableRegister, WritableRegister};

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

pub fn read_i2c_register<D, R, A, const N: usize>(
    device: &mut D,
    device_addr: A,
) -> Result<R, ReadRegisterError<D::Error, R::Error>>
where
    A: AddressMode,
    D: I2c<A>,
    R: ReadableRegister<N>,
{
    let mut buf = [0u8; N];

    device
        .write_read(device_addr, &[R::id() | 0x40], &mut buf)
        .map_err(ReadRegisterError::BusError)?;

    R::from_slice(buf).map_err(ReadRegisterError::DeserializationError)
}

pub fn write_i2c_register<D, R, A, const N: usize>(
    device: &mut D,
    device_addr: A,
    reg: R,
) -> Result<(), WriteRegisterError<D::Error, R::Error>>
where
    A: AddressMode,
    D: I2c<A>,
    R: WritableRegister<N>,
{
    let buf = reg
        .into_slice()
        .map_err(WriteRegisterError::SerializationError)?;

    device
        .transaction(
            device_addr,
            &mut [Operation::Write(&[R::id()]), Operation::Write(&buf)],
        )
        .map_err(WriteRegisterError::BusError)
}
