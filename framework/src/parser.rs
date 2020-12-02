use crate::num::PrimIntExt;
pub use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::digit1,
    combinator::opt,
    combinator::{map, map_res},
    multi::{fold_many0, fold_many1, many0, many1, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};
use num_traits::{One, Signed, Unsigned, WrappingSub};

pub type IResult<'a, T> = nom::IResult<&'a [u8], T, AocParseError<'a>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AocParseError<'a>(&'a [u8], AocErrorKind);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AocErrorKind {
    Nom(nom::error::ErrorKind),
    TakeUnsigned(&'static str),
    TakeSigned(&'static str),
    Newline,
}

impl<'a> nom::error::ParseError<&'a [u8]> for AocParseError<'a> {
    fn from_error_kind(input: &'a [u8], kind: nom::error::ErrorKind) -> Self {
        AocParseError(input, AocErrorKind::Nom(kind))
    }
    fn append(input: &'a [u8], kind: nom::error::ErrorKind, _other: Self) -> Self {
        AocParseError(input, AocErrorKind::Nom(kind))
    }
}

pub trait ParseResultToResult {
    type Output;
    fn into_result(self) -> Result<Self::Output, crate::error::Error>;
}

impl<T> ParseResultToResult for IResult<'_, T> {
    type Output = T;
    fn into_result(self) -> Result<T, crate::error::Error> {
        match self {
            Ok((remainder, output)) => {
                if remainder.is_empty() {
                    Ok(output)
                } else {
                    Err(crate::error::Error::InvalidInputDyn(format!(
                        "input not fully parsed, remainder: {:?}",
                        remainder
                    )))
                }
            }
            Err(x) => Err(crate::error::Error::InvalidInputDyn(format!("{}", x))),
        }
    }
}

pub fn newline(input: &[u8]) -> IResult<()> {
    if let Some(b'\n') = input.first() {
        Ok((&input[1..], ()))
    } else {
        Err(nom::Err::Error(AocParseError(input, AocErrorKind::Newline)))
    }
}

pub fn alpha(input: u8) -> bool {
    (input >= b'a' && input <= b'z') || (input >= b'A' && input <= b'Z')
}

pub fn digit(input: u8) -> bool {
    input >= b'0' && input <= b'9'
}

// Integer parsing

macro_rules! impl_take_uint {
    ($($ty_ident:ident),+$(,)?) => {
        $crate::paste! {
            $(
                pub fn [<take_ $ty_ident>]<'a>(input: &'a [u8]) -> IResult<'a, $ty_ident> {
                    take_unsigned::<$ty_ident>(input)
                }
            )+
        }
    }
}

macro_rules! impl_take_sint {
    ($($ty_ident:ident),+$(,)?) => {
        $crate::paste! {
            $(
                pub fn [<take_ $ty_ident>]<'a>(input: &'a [u8]) -> IResult<'a, $ty_ident> {
                    take_signed::<$ty_ident>(input)
                }
            )+
        }
    }
}

impl_take_uint!(u8, u16, u32, u64, u128, usize);
impl_take_sint!(i8, i16, i32, i64, i128, isize);

pub fn take_unsigned<T: PrimIntExt + Unsigned>(mut input: &[u8]) -> IResult<T> {
    if input.is_empty() {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeUnsigned("empty slice parsed as int"),
        )));
    }
    let overflow_error = nom::Err::Error(AocParseError(
        input,
        AocErrorKind::TakeUnsigned("overflow parsing number"),
    ));
    let mut nr = T::zero();
    while let Some(char) = input.first().cloned() {
        if char < b'0' || char > b'9' {
            break;
        }
        let digit = char - b'0';
        nr = nr
            .checked_mul(&T::from(10u8).unwrap())
            .ok_or_else(|| overflow_error.clone())?
            .checked_add(&T::from(digit).unwrap())
            .ok_or_else(|| overflow_error.clone())?;
        input = &input[1..];
    }
    Ok((input, nr))
}

fn unsigned_err_to_signed_err<'a>(err: nom::Err<AocParseError<'a>>) -> nom::Err<AocParseError<'a>> {
    err.map(|AocParseError(input, err)| {
        AocParseError(
            input,
            match err {
                AocErrorKind::TakeUnsigned(message) => AocErrorKind::TakeSigned(message),
                x => x,
            },
        )
    })
}

pub fn take_signed<T: PrimIntExt + Signed>(input: &[u8]) -> IResult<T> {
    if input.is_empty() {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeSigned("empty slice parsed as int"),
        )));
    }
    let is_negative = input[0] == b'-';
    let (remainder, mut unsigned) =
        take_unsigned::<T::Unsigned>(if is_negative { &input[1..] } else { input })
            .map_err(unsigned_err_to_signed_err)?;
    if is_negative {
        unsigned = unsigned.wrapping_sub(&T::Unsigned::one());
    }
    if unsigned >= (T::Unsigned::one() << (T::Unsigned::BITS - 1)) {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeSigned("overflow parsing number"),
        )));
    }
    if is_negative {
        unsigned = !unsigned;
    }
    return Ok((remainder, T::from_unsigned(unsigned)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_uint() {
        assert_eq!(take_u8(b"0"), Ok((&b""[..], 0u8)));
        assert_eq!(take_u8(b"128"), Ok((&b""[..], 128u8)));
        assert!(take_u8(b"256").is_err());
    }

    #[test]
    fn take_sint() {
        assert_eq!(take_i8(b"-128"), Ok((&b""[..], -128i8)));
        assert_eq!(take_i8(b"127"), Ok((&b""[..], 127i8)));
        assert!(take_i8(b"128").is_err());
    }
}
