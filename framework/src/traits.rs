use crate::error::Error;
use arrayvec::ArrayVec;

pub trait IntoResult {
    type Item;
    type Error;
    fn into_result(self) -> Result<Self::Item, Self::Error>;
}

impl<I, E> IntoResult for Result<I, E> {
    type Item = I;
    type Error = E;
    fn into_result(self) -> Self {
        self
    }
}

impl<I: IsNotResult> IntoResult for I {
    type Item = I;
    type Error = !;
    fn into_result(self) -> Result<Self, !> {
        Ok(self)
    }
}

pub auto trait IsNotResult {}
impl<T, E> !IsNotResult for Result<T, E> {}

pub trait IntoError {
    fn into_error(self) -> Error;
}

impl IntoError for ! {
    fn into_error(self) -> Error {
        unreachable!()
    }
}

impl IntoError for Error {
    fn into_error(self) -> Error {
        self
    }
}

pub trait Day {
    fn nr(&self) -> u32;
    fn evaluate(&self, input: Vec<u8>) -> ArrayVec<[(&'static str, Result<String, Error>); 2]>;
}
