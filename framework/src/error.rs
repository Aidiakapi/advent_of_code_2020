use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    IoError(#[from] std::io::Error),
    #[error("network error")]
    NetworkError,
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    #[error("invalid input: {0}")]
    InvalidInputDyn(String),
    #[error("{0:?} remainder: {1}")]
    ParseError(crate::parser::AocErrorKind, String),
    #[error("no solution found")]
    NoSolution,
}

impl From<!> for Error {
    fn from(_: !) -> Self {
        unreachable!()
    }
}
