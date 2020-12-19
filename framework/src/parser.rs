use crate::num::PrimIntExt;
pub use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, anychar, char, one_of},
    combinator::{map, map_opt, map_res, not, opt},
    multi::{fold_many0, fold_many1, many0, many1, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};
use nom::{error::ErrorKind, InputTakeAtPosition};
use num_traits::{One, Signed, Unsigned, WrappingSub};

pub type IResult<'s, T> = nom::IResult<&'s str, T, AocParseError<'s>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AocParseError<'s>(pub &'s str, pub AocErrorKind);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AocErrorKind {
    NotFullyParsed,
    Nom(ErrorKind),
    TakeUnsigned(TakeIntErrorKind),
    TakeSigned(TakeIntErrorKind),
    Whitespace,
    Char,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TakeIntErrorKind {
    Empty,
    Overflow,
    InvalidCharacter,
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
                        .map_err(|err| nom::Err::Error(AocParseError(input, AocErrorKind::TakeUnsigned(err))))
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
                        .map_err(|err| nom::Err::Error(AocParseError(input, AocErrorKind::TakeSigned(err))))
                }
            )+
        }
    }
}

impl_take_uint!(u8, u16, u32, u64, u128, usize);
impl_take_sint!(i8, i16, i32, i64, i128, isize);

fn take_unsigned<T: PrimIntExt + Unsigned>(mut input: &str) -> Result<(&str, T), TakeIntErrorKind> {
    if input.is_empty() {
        return Err(TakeIntErrorKind::Empty);
    }
    let mut nr = T::zero();
    let original_length = input.len();
    while let Some(char) = input.chars().next() {
        if char < '0' || char > '9' {
            break;
        }
        let digit = char as u8 - b'0';
        nr = nr
            .checked_mul(&T::from(10u8).unwrap())
            .ok_or(TakeIntErrorKind::Overflow)?
            .checked_add(&T::from(digit).unwrap())
            .ok_or(TakeIntErrorKind::Overflow)?;
        input = &input[1..];
    }
    if input.len() == original_length {
        Err(TakeIntErrorKind::InvalidCharacter)
    } else {
        Ok((input, nr))
    }
}

fn take_signed<T: PrimIntExt + Signed>(input: &str) -> Result<(&str, T), TakeIntErrorKind> {
    let first_char = if let Some(first_char) = input.chars().next() {
        first_char
    } else {
        return Err(TakeIntErrorKind::Empty);
    };
    let is_negative = first_char == '-';
    let (remainder, mut unsigned) =
        take_unsigned::<T::Unsigned>(if is_negative || first_char == '+' {
            &input[1..]
        } else {
            input
        })?;
    if is_negative {
        unsigned = unsigned.wrapping_sub(&T::Unsigned::one());
    }
    if unsigned >= (T::Unsigned::one() << (T::Unsigned::BITS - 1)) {
        return Err(TakeIntErrorKind::Overflow);
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
        assert_eq!(
            take_u8("256"),
            Err(nom::Err::Error(AocParseError(
                "256",
                AocErrorKind::TakeUnsigned(TakeIntErrorKind::Overflow)
            )))
        );
        assert_eq!(
            take_u8("x256"),
            Err(nom::Err::Error(AocParseError(
                "x256",
                AocErrorKind::TakeUnsigned(TakeIntErrorKind::InvalidCharacter)
            )))
        );
    }

    #[test]
    fn take_sint() {
        assert_eq!(take_i8("-128"), Ok(("", -128i8)));
        assert_eq!(take_i8("127"), Ok(("", 127i8)));
        assert_eq!(
            take_i8("128"),
            Err(nom::Err::Error(AocParseError(
                "128",
                AocErrorKind::TakeSigned(TakeIntErrorKind::Overflow)
            )))
        );
        assert_eq!(
            take_i8("x128"),
            Err(nom::Err::Error(AocParseError(
                "x128",
                AocErrorKind::TakeSigned(TakeIntErrorKind::InvalidCharacter)
            )))
        );
    }
}
