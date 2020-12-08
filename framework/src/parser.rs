use crate::num::PrimIntExt;
pub use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, anychar, char},
    combinator::{map, map_res, opt},
    multi::{fold_many0, fold_many1, many0, many1, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};
use nom::{error::ErrorKind, InputTakeAtPosition};
use num_traits::{One, Signed, Unsigned, WrappingSub};

pub type IResult<'s, T> = nom::IResult<&'s str, T, AocParseError<'s>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AocParseError<'s>(&'s str, AocErrorKind);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AocErrorKind {
    NotFullyParsed,
    Nom(ErrorKind),
    TakeUnsigned(&'static str),
    TakeSigned(&'static str),
    Whitespace,
    Char,
}

impl<'s> nom::error::ParseError<&'s str> for AocParseError<'s> {
    fn from_error_kind(input: &'s str, kind: ErrorKind) -> Self {
        AocParseError(input, AocErrorKind::Nom(kind))
    }
    fn append(input: &'s str, kind: ErrorKind, _other: Self) -> Self {
        AocParseError(input, AocErrorKind::Nom(kind))
    }
}

pub trait ParseResultToResult {
    type Output;
    fn into_result(self) -> Result<Self::Output, crate::error::Error>;
}

impl<'a, T> ParseResultToResult for IResult<'a, T> {
    type Output = T;
    fn into_result(self) -> Result<T, crate::error::Error> {
        match self {
            Ok((remainder, output)) => {
                if remainder.is_empty() {
                    Ok(output)
                } else {
                    Err(crate::error::Error::ParseError(
                        AocErrorKind::NotFullyParsed,
                        remainder.to_owned(),
                    ))
                }
            }
            Err(nom_err) => match nom_err {
                nom::Err::Incomplete(_) => panic!("do not use streaming parsing APIs"),
                nom::Err::Error(AocParseError(remainder, err))
                | nom::Err::Failure(AocParseError(remainder, err)) => {
                    Err(crate::error::Error::ParseError(err, remainder.to_owned()))
                }
            },
        }
    }
}

// pub fn newline0(input: &astr) -> IResult<&astr> {
//     split_at_position_complete(input, |item| item != achar::LineFeed)
// }

// pub fn newline1(input: &astr) -> IResult<&astr> {
//     split_at_position1_complete(input, |item| item != achar::LineFeed, ErrorKind::TakeWhile1)
// }

pub fn whitespace0(input: &str) -> IResult<&str> {
    input.split_at_position_complete(|item| !item.is_whitespace())
}

pub fn whitespace1(input: &str) -> IResult<&str> {
    input.split_at_position1_complete(|item| !item.is_ascii_whitespace(), ErrorKind::TakeWhile1)
}

// Integer parsing

macro_rules! impl_take_uint {
    ($($ty_ident:ident),+$(,)?) => {
        $crate::paste! {
            $(
                pub fn [<take_ $ty_ident>]<'a>(input: &'a str) -> IResult<'a, $ty_ident> {
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
                pub fn [<take_ $ty_ident>]<'a>(input: &'a str) -> IResult<'a, $ty_ident> {
                    take_signed::<$ty_ident>(input)
                }
            )+
        }
    }
}

impl_take_uint!(u8, u16, u32, u64, u128, usize);
impl_take_sint!(i8, i16, i32, i64, i128, isize);

pub fn take_unsigned<T: PrimIntExt + Unsigned>(mut input: &str) -> IResult<T> {
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
    while let Some(char) = input.chars().next() {
        if char < '0' || char > '9' {
            break;
        }
        let digit = char as u8 - b'0';
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

pub fn take_signed<T: PrimIntExt + Signed>(input: &str) -> IResult<T> {
    let first_char = if let Some(first_char) = input.chars().next() {
        first_char
    } else {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeSigned("empty slice parsed as int"),
        )));
    };
    let is_negative = first_char == '-';
    let (remainder, mut unsigned) =
        take_unsigned::<T::Unsigned>(if is_negative || first_char == '+' {
            &input[1..]
        } else {
            input
        })
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
        assert_eq!(take_u8("0"), Ok(("", 0u8)));
        assert_eq!(take_u8("128"), Ok(("", 128u8)));
        assert!(take_u8("256").is_err());
    }

    #[test]
    fn take_sint() {
        assert_eq!(take_i8("-128"), Ok(("", -128i8)));
        assert_eq!(take_i8("127"), Ok(("", 127i8)));
        assert!(take_i8("128").is_err());
    }
}
