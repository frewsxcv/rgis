use geo::algorithm::contains::Contains;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct CoordWithSrs<T: geo::CoordFloat> {
    pub coord: geo::Coordinate<T>,
    pub srs: String,
}

impl<T: geo::CoordFloat> CoordWithSrs<T> {
    pub fn reproject(&mut self, target_srs: &str) {
        debug_assert!(!target_srs.is_empty());

        let projector = proj::Proj::new_known_crs(&self.srs, target_srs, None).unwrap();

        self.coord = projector.convert(self.coord).unwrap();
        self.srs = target_srs.to_owned();
    }
}

#[derive(Debug, Clone)]
pub struct RectWithSrs<T: geo::CoordFloat> {
    pub rect: geo::Rect<T>,
    pub srs: String,
}

impl<T: geo::CoordFloat> RectWithSrs<T> {
    pub fn contains(&self, coord: &CoordWithSrs<T>) -> bool {
        assert_eq!(self.srs, coord.srs);
        self.rect.contains(&coord.coord)
    }

    pub fn merge(self, other: &RectWithSrs<T>) -> RectWithSrs<T> {
        assert_eq!(self.srs, other.srs);
        RectWithSrs {
            rect: geo::Rect::new(
                geo::Coordinate {
                    x: self.rect.min().x.min(other.rect.min().x),
                    y: self.rect.min().y.min(other.rect.min().y),
                },
                geo::Coordinate {
                    x: self.rect.max().x.max(other.rect.max().x),
                    y: self.rect.max().y.max(other.rect.max().y),
                },
            ),
            srs: self.srs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeometryWithSrs<T: geo::CoordFloat> {
    pub geometry: geo::Geometry<T>,
    pub srs: String,
}

impl<T: geo::CoordFloat> GeometryWithSrs<T> {
    // pub fn contains(&self, coord: &CoordWithSrs<T>) -> bool {
    //     assert_eq!(self.srs, coord.srs);
    //     // self.geometry.contains(&coord.coord)
    //     Contains::contains(&self.geometry, &coord.coord)
    // }

    pub fn reproject(&mut self, target_srs: &str) {
        self.geometry = self.geometry.transform(&self.srs, target_srs).unwrap();
        self.srs = target_srs.to_owned();
    }
}

/// Transform a Geometry from one CRS to another.
///
/// # Examples
///
/// ```
/// use geo;
/// use geo_srs::Transform;
///
/// let point: geo::Point<f32> = geo::point!(x: -36.508, y: -54.2815);
///
/// assert_eq!(
///     point.transform("EPSG:4326", "EPSG:3857").unwrap(),
///     geo::point!(x: -4064052.0, y: -7223650.5)
/// );
/// ```
pub trait Transform<T> {
    type Output;

    fn transform(&self, source_srs: &str, target: &str) -> Result<Self::Output, TransformError>;
}

#[derive(Debug)]
pub enum TransformError {
    UnknownCrs,
    ProjError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformError::UnknownCrs => write!(f, "Unknown CRS"),
            TransformError::ProjError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for TransformError {}

impl<T, G> Transform<T> for G
where
    T: geo::CoordFloat,
    G: geo::algorithm::map_coords::TryMapCoords<T, T>,
{
    type Output = G::Output;

    fn transform(
        &self,
        source_srs: &str,
        target_srs: &str,
    ) -> Result<Self::Output, TransformError> {
        let transformer = proj::Proj::new_known_crs(source_srs, target_srs, None)
            .ok_or(TransformError::UnknownCrs)?;

        self.try_map_coords(|&(x, y)| {
            transformer
                .convert((x, y))
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        })
        .map_err(|e| TransformError::ProjError(e))
    }
}
