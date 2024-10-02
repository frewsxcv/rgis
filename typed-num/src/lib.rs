#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TypedNum<Number, Type>(pub Number, std::marker::PhantomData<Type>);

impl<Scalar: geo::CoordNum, Type> TypedNum<Scalar, Type> {
    pub fn new(number: Scalar) -> TypedNum<Scalar, Type> {
        TypedNum(number, std::marker::PhantomData::<Type>)
    }
}

impl<Scalar: geo::CoordNum, Type> PartialOrd for TypedNum<Scalar, Type> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Scalar: geo::CoordNum, Type> PartialEq for TypedNum<Scalar, Type> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Scalar: geo::CoordNum, Type> std::ops::Add for TypedNum<Scalar, Type> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 + other.0)
    }
}

impl<Scalar: geo::CoordNum, Type> std::ops::Sub for TypedNum<Scalar, Type> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 - other.0)
    }
}

impl<Scalar: geo::CoordNum, Type> std::ops::Mul for TypedNum<Scalar, Type> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 * other.0)
    }
}

impl<Scalar: geo::CoordNum, Type> std::ops::Div for TypedNum<Scalar, Type> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 / other.0)
    }
}

impl<Scalar: geo::CoordNum, Type> std::ops::Rem for TypedNum<Scalar, Type> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        TypedNum::new(self.0 % other.0)
    }
}

impl<Scalar: geo::CoordNum, Type> num_traits::ToPrimitive for TypedNum<Scalar, Type> {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
}

impl<Scalar: geo::CoordNum, Type> num_traits::NumCast for TypedNum<Scalar, Type> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Scalar::from(n).map(TypedNum::new)
    }
}

impl<Scalar: geo::CoordNum, Type> num_traits::Num for TypedNum<Scalar, Type> {
    type FromStrRadixErr = Scalar::FromStrRadixErr;

    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, <Self as num_traits::Num>::FromStrRadixErr> {
        Ok(TypedNum::new(Scalar::from_str_radix(str, radix)?))
    }
}

impl<Scalar: geo::CoordNum, Type> num_traits::One for TypedNum<Scalar, Type> {
    fn one() -> Self {
        TypedNum::new(Scalar::one())
    }
}

impl<Scalar: geo::CoordNum, Type> num_traits::Zero for TypedNum<Scalar, Type> {
    fn zero() -> Self {
        TypedNum::new(Scalar::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<Scalar: num_traits::Float, Type: Clone + Copy> num_traits::Float for TypedNum<Scalar, Type> {
    fn nan() -> Self {
        TypedNum::new(Scalar::nan())
    }

    fn infinity() -> Self {
        TypedNum::new(Scalar::infinity())
    }

    fn neg_infinity() -> Self {
        TypedNum::new(Scalar::neg_infinity())
    }
}

impl<Scalar: geo::CoordFloat, Type: Clone + Copy> geo::CoordFloat for TypedNum<Scalar, Type> where
    Self: std::fmt::Debug
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[derive(Debug, Copy, Clone)]
        struct Epsg4326;

        let x: TypedNum<f64, Epsg4326> = TypedNum::new(1.0);
        let y: TypedNum<f64, Epsg4326> = TypedNum::new(2.0);
        let _point = geo::Point::new(x, y);
    }
}
