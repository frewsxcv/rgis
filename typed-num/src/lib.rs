#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Debug)]
#[repr(transparent)]
pub struct TypedNum<Number, Type>(pub Number, std::marker::PhantomData<Type>);

impl<Scalar: Clone, Type> Clone for TypedNum<Scalar, Type> {
    fn clone(&self) -> Self {
        TypedNum(self.0.clone(), self.1)
    }
}

impl<Scalar: Copy, Type> Copy for TypedNum<Scalar, Type> {}

impl<Scalar: Default, Type> Default for TypedNum<Scalar, Type> {
    fn default() -> Self {
        TypedNum(Scalar::default(), std::marker::PhantomData::<Type>)
    }
}

impl<Scalar, Type> From<Scalar> for TypedNum<Scalar, Type> {
    fn from(number: Scalar) -> Self {
        TypedNum(number, std::marker::PhantomData::<Type>)
    }
}

impl<Scalar, Type> TypedNum<Scalar, Type> {
    #[inline]
    pub fn new(number: Scalar) -> TypedNum<Scalar, Type> {
        TypedNum(number, std::marker::PhantomData::<Type>)
    }
}

impl<Scalar: PartialOrd, Type> PartialOrd for TypedNum<Scalar, Type> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Scalar: PartialEq, Type> PartialEq for TypedNum<Scalar, Type> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Scalar: std::cmp::Eq, Type> std::cmp::Eq for TypedNum<Scalar, Type> {}

impl<Scalar: Add<Output = Scalar>, Type> Add for TypedNum<Scalar, Type> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 + other.0)
    }
}

impl<Scalar: Sub<Output = Scalar>, Type> Sub for TypedNum<Scalar, Type> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 - other.0)
    }
}

impl<Scalar: Mul<Output = Scalar>, Type> Mul for TypedNum<Scalar, Type> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 * other.0)
    }
}

impl<Scalar: Div<Output = Scalar>, Type> Div for TypedNum<Scalar, Type> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 / other.0)
    }
}

impl<Scalar: Rem<Output = Scalar>, Type> Rem for TypedNum<Scalar, Type> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 % other.0)
    }
}

impl<Scalar: AddAssign, Type> AddAssign for TypedNum<Scalar, Type> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl<Scalar: SubAssign, Type> SubAssign for TypedNum<Scalar, Type> {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl<Scalar: MulAssign, Type> MulAssign for TypedNum<Scalar, Type> {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

impl<Scalar: DivAssign, Type> DivAssign for TypedNum<Scalar, Type> {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}

impl<Scalar: RemAssign, Type> RemAssign for TypedNum<Scalar, Type> {
    fn rem_assign(&mut self, other: Self) {
        self.0 %= other.0;
    }
}

impl<Scalar: num_traits::ToPrimitive, Type> num_traits::ToPrimitive for TypedNum<Scalar, Type> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }
}

impl<Scalar: num_traits::NumCast, Type> num_traits::NumCast for TypedNum<Scalar, Type> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Scalar::from(n).map(TypedNum::new)
    }
}

impl<Scalar: num_traits::Num, Type> num_traits::Num for TypedNum<Scalar, Type> {
    type FromStrRadixErr = Scalar::FromStrRadixErr;

    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, <Self as num_traits::Num>::FromStrRadixErr> {
        Ok(TypedNum::new(Scalar::from_str_radix(str, radix)?))
    }
}

impl<Scalar: num_traits::One, Type> num_traits::One for TypedNum<Scalar, Type> {
    fn one() -> Self {
        TypedNum::new(Scalar::one())
    }
}

impl<Scalar: num_traits::Zero, Type> num_traits::Zero for TypedNum<Scalar, Type> {
    fn zero() -> Self {
        TypedNum::new(Scalar::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<Scalar: num_traits::Float, Type> num_traits::Float for TypedNum<Scalar, Type>
where
    Self: std::ops::Neg<Output = Self> + std::fmt::Debug,
{
    fn nan() -> Self {
        TypedNum::new(Scalar::nan())
    }

    fn infinity() -> Self {
        TypedNum::new(Scalar::infinity())
    }

    fn neg_infinity() -> Self {
        TypedNum::new(Scalar::neg_infinity())
    }

    fn neg_zero() -> Self {
        TypedNum::new(Scalar::neg_zero())
    }

    fn min_value() -> Self {
        TypedNum::new(Scalar::min_value())
    }

    fn min_positive_value() -> Self {
        TypedNum::new(Scalar::min_positive_value())
    }

    fn max_value() -> Self {
        TypedNum::new(Scalar::max_value())
    }

    fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.0.classify()
    }

    fn floor(self) -> Self {
        TypedNum::new(self.0.floor())
    }

    fn ceil(self) -> Self {
        TypedNum::new(self.0.ceil())
    }

    fn round(self) -> Self {
        TypedNum::new(self.0.round())
    }

    fn trunc(self) -> Self {
        TypedNum::new(self.0.trunc())
    }

    fn fract(self) -> Self {
        TypedNum::new(self.0.fract())
    }

    fn abs(self) -> Self {
        TypedNum::new(self.0.abs())
    }

    fn signum(self) -> Self {
        TypedNum::new(self.0.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        TypedNum::new(self.0.mul_add(a.0, b.0))
    }

    fn recip(self) -> Self {
        TypedNum::new(self.0.recip())
    }

    fn powi(self, n: i32) -> Self {
        TypedNum::new(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        TypedNum::new(self.0.powf(n.0))
    }

    fn sqrt(self) -> Self {
        TypedNum::new(self.0.sqrt())
    }

    fn exp(self) -> Self {
        TypedNum::new(self.0.exp())
    }

    fn exp2(self) -> Self {
        TypedNum::new(self.0.exp2())
    }

    fn ln(self) -> Self {
        TypedNum::new(self.0.ln())
    }

    fn log(self, base: Self) -> Self {
        TypedNum::new(self.0.log(base.0))
    }

    fn log2(self) -> Self {
        TypedNum::new(self.0.log2())
    }

    fn log10(self) -> Self {
        TypedNum::new(self.0.log10())
    }

    fn max(self, other: Self) -> Self {
        TypedNum::new(self.0.max(other.0))
    }

    fn min(self, other: Self) -> Self {
        TypedNum::new(self.0.min(other.0))
    }

    fn abs_sub(self, other: Self) -> Self {
        TypedNum::new(self.0.abs_sub(other.0))
    }

    fn cbrt(self) -> Self {
        TypedNum::new(self.0.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        TypedNum::new(self.0.hypot(other.0))
    }

    fn sin(self) -> Self {
        TypedNum::new(self.0.sin())
    }

    fn cos(self) -> Self {
        TypedNum::new(self.0.cos())
    }

    fn tan(self) -> Self {
        TypedNum::new(self.0.tan())
    }

    fn asin(self) -> Self {
        TypedNum::new(self.0.asin())
    }

    fn acos(self) -> Self {
        TypedNum::new(self.0.acos())
    }

    fn atan(self) -> Self {
        TypedNum::new(self.0.atan())
    }

    fn atan2(self, other: Self) -> Self {
        TypedNum::new(self.0.atan2(other.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (TypedNum::new(sin), TypedNum::new(cos))
    }

    fn exp_m1(self) -> Self {
        TypedNum::new(self.0.exp_m1())
    }

    fn ln_1p(self) -> Self {
        TypedNum::new(self.0.ln_1p())
    }

    fn sinh(self) -> Self {
        TypedNum::new(self.0.sinh())
    }

    fn cosh(self) -> Self {
        TypedNum::new(self.0.cosh())
    }

    fn tanh(self) -> Self {
        TypedNum::new(self.0.tanh())
    }

    fn asinh(self) -> Self {
        TypedNum::new(self.0.asinh())
    }

    fn acosh(self) -> Self {
        TypedNum::new(self.0.acosh())
    }

    fn atanh(self) -> Self {
        TypedNum::new(self.0.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
}

impl<Scalar: float_next_after::NextAfter, Type> float_next_after::NextAfter
    for TypedNum<Scalar, Type>
{
    fn next_after(self, other: Self) -> Self {
        TypedNum::new(self.0.next_after(other.0))
    }
}

impl<Scalar: num_traits::Bounded, Type> num_traits::Bounded for TypedNum<Scalar, Type> {
    fn min_value() -> Self {
        TypedNum::new(Scalar::min_value())
    }

    fn max_value() -> Self {
        TypedNum::new(Scalar::max_value())
    }
}

impl<Scalar: std::ops::Neg<Output = Scalar>, Type> std::ops::Neg for TypedNum<Scalar, Type> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        TypedNum::new(-self.0)
    }
}

impl<Scalar: num_traits::Signed + std::ops::Neg<Output = Scalar> + num_traits::Num, Type>
    num_traits::Signed for TypedNum<Scalar, Type>
where
    Self: std::ops::Neg<Output = Self>,
{
    fn abs(&self) -> Self {
        TypedNum::new(self.0.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        TypedNum::new(self.0.abs_sub(&other.0))
    }

    fn signum(&self) -> Self {
        TypedNum::new(self.0.signum())
    }

    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<Scalar: geo::GeoNum, Type: std::fmt::Debug> geo::GeoNum for TypedNum<Scalar, Type>
where
    Self: std::ops::Neg<Output = Self>,
    <Scalar as geo::GeoNum>::Ker: geo::Kernel<TypedNum<Scalar, Type>>,
{
    type Ker = <Scalar as geo::GeoNum>::Ker;

    fn total_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl<Scalar: FromStr, Type> FromStr for TypedNum<Scalar, Type> {
    type Err = Scalar::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TypedNum::new(Scalar::from_str(s)?))
    }
}

impl<Scalar: num_traits::FromPrimitive, Type> num_traits::FromPrimitive for TypedNum<Scalar, Type> {
    fn from_i64(n: i64) -> Option<Self> {
        Scalar::from_i64(n).map(TypedNum::new)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Scalar::from_u64(n).map(TypedNum::new)
    }
}

impl<Scalar: std::cmp::Ord, Type> std::cmp::Ord for TypedNum<Scalar, Type> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<Scalar: std::fmt::Display, Type> std::fmt::Display for TypedNum<Scalar, Type> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Scalar: Sum, Type> Sum for TypedNum<Scalar, Type> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        TypedNum::new(iter.map(|n| n.0).sum())
    }
}
