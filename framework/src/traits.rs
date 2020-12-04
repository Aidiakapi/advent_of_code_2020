use crate::{error::Error, AString};
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

// FIXME: Change this to return the never type when issues with this are fix
impl<I: IsNotResult> IntoResult for I {
    type Item = I;
    type Error = crate::error::Error;
    fn into_result(self) -> Result<Self, crate::error::Error> {
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
    fn evaluate(&self, input: AString) -> ArrayVec<[(&'static str, Result<String, Error>); 2]>;
}
