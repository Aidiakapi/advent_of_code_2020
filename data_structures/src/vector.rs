#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Construction
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::new($x, $y)
    };
}
#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x, $y, $z)
    };
}

impl<T> Vec2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}
impl<T> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

// Vector type aliases
macro_rules! vec_aliases {
    ($($underlying:ident)+) => {
        paste::paste! {
            $(
                #[allow(non_camel_case_types)]
                pub type [<$underlying x2>] = Vec2<$underlying>;
                #[allow(non_camel_case_types)]
                pub type [<$underlying x3>] = Vec3<$underlying>;
            )+
        }
    };
}

vec_aliases! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

// Vector specific functions
pub trait DotProduct: Clone {
    type Output;
    fn dot(self, rhs: Self) -> Self::Output;

    #[inline]
    fn magnitude_sqr(self) -> Self::Output {
        self.clone().dot(self)
    }
}

pub trait Magnitude: DotProduct {
    fn magnitude(self) -> Self::Output;
}

impl<T> DotProduct for Vec2<T>
where
    T: Clone + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    type Output = T;

    #[inline]
    fn dot(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<T> DotProduct for Vec3<T>
where
    T: Clone + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    type Output = T;

    #[inline]
    fn dot(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z
    }
}

impl<T> Magnitude for T
where
    T: DotProduct,
    T::Output: num::Float,
{
    #[inline]
    fn magnitude(self) -> Self::Output {
        num::Float::sqrt(self.magnitude_sqr())
    }
}

// Operators
macro_rules! impl_binary_ops {
    ($($symbol:tt $op_name:ident $fn_name:ident $assign_symbol:tt $assign_op_name:ident $assign_fn_name:ident,)+) => {
        $(
            impl<T, U> std::ops::$op_name for Vec2<T>
            where T: std::ops::$op_name<Output = U>
            {
                type Output = Vec2<U>;

                #[inline]
                fn $fn_name(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        x: self.x $symbol rhs.x,
                        y: self.y $symbol rhs.y,
                    }
                }
            }

            impl<T, U> std::ops::$op_name for Vec3<T>
            where T: std::ops::$op_name<Output = U>
            {
                type Output = Vec3<U>;

                #[inline]
                fn $fn_name(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        x: self.x $symbol rhs.x,
                        y: self.y $symbol rhs.y,
                        z: self.z $symbol rhs.z,
                    }
                }
            }

            impl<T> std::ops::$assign_op_name for Vec2<T>
            where
                T: std::ops::$assign_op_name,
            {
                #[inline]
                fn $assign_fn_name(&mut self, rhs: Self) {
                    self.x $assign_symbol rhs.x;
                    self.y $assign_symbol rhs.y;
                }
            }
        )+
    };
}

macro_rules! impl_unary_ops {
    ($($symbol:tt $op_name:ident $fn_name:ident,)+) => {
        $(
            impl<T> std::ops::$op_name for Vec2<T>
            where
                T: std::ops::$op_name,
            {
                type Output = Vec2<T::Output>;

                #[inline]
                fn $fn_name(self) -> Self::Output {
                    Self::Output {
                        x: $symbol self.x,
                        y: $symbol self.y,
                    }
                }
            }

            impl<T> std::ops::$op_name for Vec3<T>
            where
                T: std::ops::$op_name,
            {
                type Output = Vec3<T::Output>;

                #[inline]
                fn $fn_name(self) -> Self::Output {
                    Self::Output {
                        x: $symbol self.x,
                        y: $symbol self.y,
                        z: $symbol self.z,
                    }
                }
            }
        )+
    };
}

impl_binary_ops! {
    +  Add    add      +=  AddAssign    add_assign,
    -  Sub    sub      -=  SubAssign    sub_assign,
    *  Mul    mul      *=  MulAssign    mul_assign,
    /  Div    div      /=  DivAssign    div_assign,
    %  Rem    rem      %=  RemAssign    rem_assign,
    &  BitAnd bitand   &=  BitAndAssign bitand_assign,
    |  BitOr  bitor    |=  BitOrAssign  bitor_assign,
    ^  BitXor bitxor   ^=  BitXorAssign bitxor_assign,
    << Shl    shl      <<= ShlAssign    shl_assign,
    >> Shr    shr      >>= ShrAssign    shr_assign,
}

impl_unary_ops! {
    - Neg neg,
    ! Not not,
}

// Tuple conversions
impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(Vec2 { x, y }: Vec2<T>) -> Self {
        (x, y)
    }
}

impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(Vec3 { x, y, z }: Vec3<T>) -> Self {
        (x, y, z)
    }
}

// Int vector extensions
pub trait IntVec2: Sized {
    /// Gets the neighbors in the cardinal directions: N, E, S, W
    fn neighbors_cardinal(self) -> [Self; 4];
    /// Gets the neighbors in the ordinal directions: NE, SE, SW, NW
    fn neighbors_ordinal(self) -> [Self; 4];
    /// Gets the neighbors in both the ordinal and cardinal directions:
    /// N, NE, E, SE, S, SW, W, NW
    fn neighbors(self) -> [Self; 8];
}

impl<T> IntVec2 for Vec2<T>
where
    T : num::PrimInt
{
    fn neighbors_cardinal(self) -> [Self; 4] {
        [
            Self::new(self.x, self.y + T::one()),
            Self::new(self.x + T::one(), self.y),
            Self::new(self.x, self.y - T::one()),
            Self::new(self.x - T::one(), self.y),
        ]
    }

    fn neighbors_ordinal(self) -> [Self; 4] {
        [
            Self::new(self.x + T::one(), self.y + T::one()),
            Self::new(self.x + T::one(), self.y - T::one()),
            Self::new(self.x - T::one(), self.y - T::one()),
            Self::new(self.x - T::one(), self.y + T::one()),
        ]
    }

    fn neighbors(self) -> [Self; 8] {
        [
            Self::new(self.x, self.y + T::one()),
            Self::new(self.x + T::one(), self.y + T::one()),
            Self::new(self.x + T::one(), self.y),
            Self::new(self.x + T::one(), self.y - T::one()),
            Self::new(self.x, self.y - T::one()),
            Self::new(self.x - T::one(), self.y - T::one()),
            Self::new(self.x - T::one(), self.y),
            Self::new(self.x - T::one(), self.y + T::one()),
        ]
    }
}
