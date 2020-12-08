pub use crate::ascii::*;
use crate::num::PrimIntExt;
pub use nom::{
    branch::alt,
    combinator::{map, map_res, opt},
    multi::{fold_many0, fold_many1, many0, many1, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};
use num_traits::{One, Signed, Unsigned, WrappingSub};

use nom::error::ErrorKind;

pub type IResult<'a, T> = nom::IResult<&'a astr, T, AocParseError<'a>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AocParseError<'a>(&'a astr, AocErrorKind);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AocErrorKind {
    NotFullyParsed,
    Nom(ErrorKind),
    TakeUnsigned(&'static str),
    TakeSigned(&'static str),
    Whitespace,
    Char,
}

impl<'a> nom::error::ParseError<&'a astr> for AocParseError<'a> {
    fn from_error_kind(input: &'a astr, kind: ErrorKind) -> Self {
        AocParseError(input, AocErrorKind::Nom(kind))
    }
    fn append(input: &'a astr, kind: ErrorKind, _other: Self) -> Self {
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

pub fn take(count: usize) -> impl Fn(&astr) -> IResult<&astr> {
    move |input| {
        if input.len() >= count {
            Ok((&input[count..], &input[0..count]))
        } else {
            Err(nom::Err::Error(AocParseError(
                input,
                AocErrorKind::Nom(ErrorKind::Eof),
            )))
        }
    }
}

pub fn anychar(input: &astr) -> IResult<achar> {
    if let Some(char) = input.first() {
        Ok((&input[1..], char))
    } else {
        Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::Nom(ErrorKind::Eof),
        )))
    }
}

pub fn char(expected: achar) -> impl Fn(&astr) -> IResult<achar> {
    move |input| {
        if let Some(char) = input.first() {
            if char == expected {
                return Ok((&input[1..], char));
            }
        }
        Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::Nom(ErrorKind::Char),
        )))
    }
}

pub fn tag<'a, 'b>(expected: &'a astr) -> impl Fn(&'b astr) -> IResult<&'b astr>
where
    'a: 'b,
{
    move |input| {
        if input.len() < expected.len() {
            Err(nom::Err::Error(AocParseError(
                input,
                AocErrorKind::Nom(ErrorKind::Eof),
            )))
        } else if &input[0..expected.len()] == expected {
            Ok((&input[expected.len()..], &input[0..expected.len()]))
        } else {
            Err(nom::Err::Error(AocParseError(
                input,
                AocErrorKind::Nom(ErrorKind::Tag),
            )))
        }
    }
}

pub fn take_while1<P>(predicate: P) -> impl Fn(&astr) -> IResult<&astr>
where
    P: Fn(achar) -> bool,
{
    move |input| {
        let count = input.chars().take_while(|&char| predicate(char)).count();
        if count == 0 {
            Err(nom::Err::Error(AocParseError(
                input,
                AocErrorKind::Nom(ErrorKind::TakeWhile1),
            )))
        } else {
            Ok((&input[count..], &input[0..count]))
        }
    }
}

fn split_at_position_complete<P>(input: &astr, predicate: P) -> IResult<&astr>
where
    P: Fn(achar) -> bool,
{
    use nom::InputTakeAtPosition;
    unsafe {
        input
            .as_bytes()
            .split_at_position_complete(|item| predicate(achar::from_ascii_unchecked(item)))
            .map(|(remainder, result)| {
                (
                    astr::from_ascii_unchecked(remainder),
                    astr::from_ascii_unchecked(result),
                )
            })
            .map_err(|err: nom::Err<(&[u8], ErrorKind)>| {
                err.map(|err| {
                    AocParseError(astr::from_ascii_unchecked(err.0), AocErrorKind::Nom(err.1))
                })
            })
    }
}

fn split_at_position1_complete<P>(
    input: &astr,
    predicate: P,
    error_kind: ErrorKind,
) -> IResult<&astr>
where
    P: Fn(achar) -> bool,
{
    use nom::InputTakeAtPosition;
    unsafe {
        input
            .as_bytes()
            .split_at_position1_complete(
                |item| predicate(achar::from_ascii_unchecked(item)),
                error_kind,
            )
            .map(|(remainder, result)| {
                (
                    astr::from_ascii_unchecked(remainder),
                    astr::from_ascii_unchecked(result),
                )
            })
            .map_err(|err: nom::Err<(&[u8], ErrorKind)>| {
                err.map(|err| {
                    AocParseError(astr::from_ascii_unchecked(err.0), AocErrorKind::Nom(err.1))
                })
            })
    }
}

pub fn newline0(input: &astr) -> IResult<&astr> {
    split_at_position_complete(input, |item| item != achar::LineFeed)
}

pub fn newline1(input: &astr) -> IResult<&astr> {
    split_at_position1_complete(input, |item| item != achar::LineFeed, ErrorKind::TakeWhile1)
}

pub fn alpha0(input: &astr) -> IResult<&astr> {
    split_at_position_complete(input, |item| !item.is_alphabetic())
}

pub fn alpha1(input: &astr) -> IResult<&astr> {
    split_at_position1_complete(input, |item| !item.is_ascii_alphabetic(), ErrorKind::Alpha)
}

pub fn digit0(input: &astr) -> IResult<&astr> {
    split_at_position_complete(input, |item| !item.is_ascii_digit())
}

pub fn digit1(input: &astr) -> IResult<&astr> {
    split_at_position1_complete(input, |item| !item.is_ascii_digit(), ErrorKind::Digit)
}

pub fn whitespace0(input: &astr) -> IResult<&astr> {
    split_at_position_complete(input, |item| !item.is_whitespace())
}

pub fn whitespace1(input: &astr) -> IResult<&astr> {
    split_at_position1_complete(
        input,
        |item| !item.is_ascii_whitespace(),
        ErrorKind::TakeWhile1,
    )
}

// Integer parsing

macro_rules! impl_take_uint {
    ($($ty_ident:ident),+$(,)?) => {
        $crate::paste! {
            $(
                pub fn [<take_ $ty_ident>]<'a>(input: &'a astr) -> IResult<'a, $ty_ident> {
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
                pub fn [<take_ $ty_ident>]<'a>(input: &'a astr) -> IResult<'a, $ty_ident> {
                    take_signed::<$ty_ident>(input)
                }
            )+
        }
    }
}

impl_take_uint!(u8, u16, u32, u64, u128, usize);
impl_take_sint!(i8, i16, i32, i64, i128, isize);

pub fn take_unsigned<T: PrimIntExt + Unsigned>(mut input: &astr) -> IResult<T> {
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
    while let Some(char) = input.first() {
        if char < b'0' || char > b'9' {
            break;
        }
        let digit = char.as_byte() - b'0';
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

pub fn take_signed<T: PrimIntExt + Signed>(input: &astr) -> IResult<T> {
    if input.is_empty() {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeSigned("empty slice parsed as int"),
        )));
    }
    let is_negative = input[0] == b'-';
    let (remainder, mut unsigned) =
        take_unsigned::<T::Unsigned>(if is_negative || input[0] == b'+' {
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

    macro_rules! empty_str {
        () => {
            astr::from_ascii(b"").unwrap()
        };
    }

    #[test]
    fn take_uint() {
        assert_eq!(
            take_u8(astr::from_ascii(b"0").unwrap()),
            Ok((empty_str!(), 0u8))
        );
        assert_eq!(
            take_u8(astr::from_ascii(b"128").unwrap()),
            Ok((empty_str!(), 128u8))
        );
        assert!(take_u8(astr::from_ascii(b"256").unwrap()).is_err());
    }

    #[test]
    fn take_sint() {
        assert_eq!(
            take_i8(astr::from_ascii(b"-128").unwrap()),
            Ok((empty_str!(), -128i8))
        );
        assert_eq!(
            take_i8(astr::from_ascii(b"127").unwrap()),
            Ok((empty_str!(), 127i8))
        );
        assert!(take_i8(astr::from_ascii(b"128").unwrap()).is_err());
    }
}
