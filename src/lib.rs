pub use from_slice::*;
pub use into_slice::*;

pub mod embedded_hal;
mod from_slice;
mod into_slice;

/// The generic top level trait for all register values
pub trait Register {
    fn id() -> u8;
}

/// A marker trait for registers that are readable
pub trait ReadableRegister<const N: usize = 1>: Register + FromSlice<N> {}

/// A marker trait for registers that are writable
pub trait WritableRegister<const N: usize = 1>: Register + IntoSlice<N> {}
