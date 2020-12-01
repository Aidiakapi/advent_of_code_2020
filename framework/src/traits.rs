use crate::error::Error;
use arrayvec::ArrayVec;
use std::fmt::Display;

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

impl<
        FParser,
        FPt1,
        FPt2,
        Input,
        Pt1Item,
        Pt2Item,
        ParserErr,
        Pt1Err,
        Pt2Err,
        ParserOut,
        Pt1Out,
        Pt2Out,
    > Day
    for (
        u32,
        &'static str,
        FParser,
        &'static str,
        FPt1,
        &'static str,
        FPt2,
    )
where
    FParser: Fn(&[u8]) -> ParserOut,
    FPt1: Fn(&Input) -> Pt1Out,
    FPt2: Fn(&Input) -> Pt2Out,
    Pt1Item: Display,
    Pt2Item: Display,
    ParserErr: IntoError,
    Pt1Err: IntoError,
    Pt2Err: IntoError,
    ParserOut: IntoResult<Item = Input, Error = ParserErr>,
    Pt1Out: IntoResult<Item = Pt1Item, Error = Pt1Err>,
    Pt2Out: IntoResult<Item = Pt2Item, Error = Pt2Err>,
{
    fn nr(&self) -> u32 {
        self.0
    }

    fn evaluate(&self, input: Vec<u8>) -> ArrayVec<[(&'static str, Result<String, Error>); 2]> {
        let mut res = ArrayVec::new();
        let input = match (self.2)(&input).into_result() {
            Ok(input) => input,
            Err(err) => {
                res.push((self.1, Err(err.into_error())));
                return res;
            }
        };

        res.push((
            self.3,
            self.4(&input)
                .into_result()
                .map(|x| x.to_string())
                .map_err(|x| x.into_error()),
        ));
        res.push((
            self.5,
            self.6(&input)
                .into_result()
                .map(|x| x.to_string())
                .map_err(|x| x.into_error()),
        ));
        res
    }
}
