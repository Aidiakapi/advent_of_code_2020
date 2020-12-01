pub use framework::{
    day,
    error::{Error, Result},
    parser, standard_tests,
};


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
