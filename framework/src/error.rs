use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    #[error("invalid input: {0}")]
    InvalidInputDyn(String),
    #[error("no solution found")]
    NoSolution,
}
