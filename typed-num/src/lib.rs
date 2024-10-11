#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use std::str::FromStr;

use num_traits::float;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TypedNum<Number, Type>(pub Number, std::marker::PhantomData<Type>);

impl<Scalar: Default, Type> Default for TypedNum<Scalar, Type> {
    fn default() -> Self {
        TypedNum(Scalar::default(), std::marker::PhantomData::<Type>)
    }
}

impl<Scalar: num_traits::Num, Type> From<Scalar> for TypedNum<Scalar, Type> {
    fn from(number: Scalar) -> Self {
        TypedNum(number, std::marker::PhantomData::<Type>)
    }
}

impl<Scalar: num_traits::Num, Type> TypedNum<Scalar, Type> {
    pub fn new(number: Scalar) -> TypedNum<Scalar, Type> {
        TypedNum(number, std::marker::PhantomData::<Type>)
    }
}

impl<Scalar: num_traits::Float, Type> PartialOrd for TypedNum<Scalar, Type> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Scalar: num_traits::Num, Type> PartialEq for TypedNum<Scalar, Type> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Scalar: num_traits::Num, Type> std::ops::Add for TypedNum<Scalar, Type> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 + other.0)
    }
}

impl<Scalar: num_traits::Num, Type> std::ops::Sub for TypedNum<Scalar, Type> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 - other.0)
    }
}

impl<Scalar: num_traits::Num, Type> std::ops::Mul for TypedNum<Scalar, Type> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 * other.0)
    }
}

impl<Scalar: num_traits::Num, Type> std::ops::Div for TypedNum<Scalar, Type> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 / other.0)
    }
}

impl<Scalar: num_traits::Num, Type> std::ops::Rem for TypedNum<Scalar, Type> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 % other.0)
    }
}

impl<Scalar: num_traits::ToPrimitive, Type> num_traits::ToPrimitive for TypedNum<Scalar, Type> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
}

impl<Scalar: num_traits::Num + num_traits::NumCast, Type> num_traits::NumCast
    for TypedNum<Scalar, Type>
{
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Scalar::from(n).map(TypedNum::new)
    }
}

impl<Scalar: num_traits::Num + num_traits::NumCast, Type> num_traits::Num
    for TypedNum<Scalar, Type>
{
    type FromStrRadixErr = Scalar::FromStrRadixErr;

    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, <Self as num_traits::Num>::FromStrRadixErr> {
        Ok(TypedNum::new(Scalar::from_str_radix(str, radix)?))
    }
}

impl<Scalar: num_traits::Num + num_traits::One, Type> num_traits::One for TypedNum<Scalar, Type> {
    fn one() -> Self {
        TypedNum::new(Scalar::one())
    }
}

impl<Scalar: num_traits::Zero + num_traits::Num, Type> num_traits::Zero for TypedNum<Scalar, Type> {
    fn zero() -> Self {
        TypedNum::new(Scalar::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<Scalar: num_traits::Float, Type: Clone + Copy> num_traits::Float for TypedNum<Scalar, Type>
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

// TODO: put behind feature flag?
impl<Scalar: float_next_after::NextAfter + num_traits::Num, Type> float_next_after::NextAfter
    for TypedNum<Scalar, Type>
{
    fn next_after(self, other: Self) -> Self {
        TypedNum::new(self.0.next_after(other.0))
    }
}

impl<Scalar: num_traits::Bounded + num_traits::Num, Type> num_traits::Bounded
    for TypedNum<Scalar, Type>
{
    fn min_value() -> Self {
        TypedNum::new(Scalar::min_value())
    }

    fn max_value() -> Self {
        TypedNum::new(Scalar::max_value())
    }
}

impl<Scalar: num_traits::Signed + std::ops::Neg<Output = Scalar> + num_traits::NumCast, Type>
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

// impl<Scalar: geo::GeoFloat + std::ops::Neg<Output = Scalar>, Type: std::fmt::Debug + Copy>
//     geo::GeoFloat for TypedNum<Scalar, Type>
// where
//     Self: geo::GeoNum + std::ops::Neg<Output = Self>,
// {
// }

impl<Scalar: FromStr + num_traits::Num, Type> FromStr for TypedNum<Scalar, Type> {
    type Err = Scalar::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TypedNum::new(Scalar::from_str(s)?))
    }
}
