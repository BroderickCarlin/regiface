use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum ReadRegisterError<B, D> {
    BusError(B),
    DeserializationError(D),
}

#[derive(Clone, Copy, Debug, Error)]
pub enum WriteRegisterError<B, S> {
    BusError(B),
    SerializationError(S),
}
