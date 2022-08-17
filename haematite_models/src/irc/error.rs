use serde::ser::Error as SerError;

#[derive(Debug)]
pub enum Error {
    Serialize,

    UnknownBan,
    UnknownChannel,
    UnknownServer,
    UnknownUser,
    UnknownMode,
}

impl<E: SerError> From<E> for Error {
    fn from(_error: E) -> Self {
        Self::Serialize
    }
}
