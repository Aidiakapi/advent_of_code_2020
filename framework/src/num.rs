use num_traits::{ops::wrapping::*, PrimInt, Signed, Unsigned};

pub trait PrimIntExt: PrimInt + std::fmt::Display {
    type Signed: PrimIntExt
        + Signed
        + WrappingAdd
        + WrappingSub
        + WrappingMul
        + WrappingNeg
        + WrappingShl
        + WrappingShr;
    type Unsigned: PrimIntExt
        + Unsigned
        + WrappingAdd
        + WrappingSub
        + WrappingMul
        + WrappingNeg
        + WrappingShl
        + WrappingShr;
    const BITS: usize;
    fn as_signed(self) -> Self::Signed;
    fn as_unsigned(self) -> Self::Unsigned;
    fn from_signed(value: Self::Signed) -> Self;
    fn from_unsigned(value: Self::Unsigned) -> Self;
}

macro_rules! impl_prim_int_ext {
    ($(($unsigned:ty, $signed:ty),)+) => {
        $(
            impl PrimIntExt for $unsigned {
                type Signed = $signed;
                type Unsigned = $unsigned;
                const BITS: usize = std::mem::size_of::<$unsigned>() * 8;
                #[inline(always)]
                fn as_signed(self) -> Self::Signed { self as $signed }
                #[inline(always)]
                fn as_unsigned(self) -> Self::Unsigned { self }
                #[inline(always)]
                fn from_signed(value: Self::Signed) -> Self { value as $unsigned }
                #[inline(always)]
                fn from_unsigned(value: Self::Unsigned) -> Self { value }
            }
            impl PrimIntExt for $signed {
                type Signed = $signed;
                type Unsigned = $unsigned;
                const BITS: usize = std::mem::size_of::<$unsigned>() * 8;
                #[inline(always)]
                fn as_signed(self) -> Self::Signed { self }
                #[inline(always)]
                fn as_unsigned(self) -> Self::Unsigned { self as $unsigned }
                #[inline(always)]
                fn from_signed(value: Self::Signed) -> Self { value }
                #[inline(always)]
                fn from_unsigned(value: Self::Unsigned) -> Self { value as $signed }
            }
        )+
    };
}

impl_prim_int_ext!(
    (u8, i8),
    (u16, i16),
    (u32, i32),
    (u64, i64),
    (u128, i128),
    (usize, isize),
);
