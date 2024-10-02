#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use typed_num::TypedNum;

#[derive(Debug, Copy, Clone)]
pub struct Projected;

#[derive(Debug, Copy, Clone)]
pub struct Unprojected;

pub type UnprojectedScalar = TypedNum<f64, Unprojected>;
pub type ProjectedScalar = TypedNum<f64, Projected>;

pub type ProjectedCoord<T = f64> = geo::Coord<TypedNum<T, Projected>>;
pub type UnprojectedCoord<T = f64> = geo::Coord<TypedNum<T, Unprojected>>;
