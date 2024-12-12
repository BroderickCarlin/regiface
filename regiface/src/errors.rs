#[derive(Clone, Copy, Debug)]
pub enum ReadRegisterError<B, D> {
    BusError(B),
    DeserializationError(D),
}

#[derive(Clone, Copy, Debug)]
pub enum WriteRegisterError<B, S> {
    BusError(B),
    SerializationError(S),
}

#[derive(Clone, Copy, Debug)]
pub enum CommandError<B, S, D> {
    BusError(B),
    SerializationError(S),
    DeserializationError(D),
}
