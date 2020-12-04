use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid ascii in input: {0}")]
    InvalidAscii(#[from] ascii::AsAsciiStrError),
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    #[error("invalid input: {0}")]
    InvalidInputDyn(String),
    #[error("{0:?} remainder: {1}")]
    ParseError(crate::parser::AocErrorKind, crate::ascii::AString),
    #[error("no solution found")]
    NoSolution,
}

impl From<!> for Error {
    fn from(_: !) -> Self {
        unreachable!()
    }
}
