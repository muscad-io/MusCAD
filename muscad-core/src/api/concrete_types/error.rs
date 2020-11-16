use super::*;

pub enum Error {
    NotEnoughPoints,
}

impl From<sealed::OperationError<sealed::NotEnoughPoints>> for Error {
    fn from(_: sealed::OperationError<sealed::NotEnoughPoints>) -> Self {
        Error::NotEnoughPoints
    }
}
