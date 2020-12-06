pub use framework::{
    ascii::{achar, astr, AString, AsciiStrExt},
    astr, day,
    error::{Error, Result},
    iter::*,
    parser, standard_tests,
};
pub use itertools::Itertools;
pub use lazy_static::lazy_static;

pub trait ResultWhereValueIsErrorExt {
    type Type;
    fn unwrap_either(self) -> Self::Type;
}

impl<T> ResultWhereValueIsErrorExt for std::result::Result<T, T> {
    type Type = T;
    fn unwrap_either(self) -> T {
        match self {
            Ok(x) => x,
            Err(x) => x,
        }
    }
}
