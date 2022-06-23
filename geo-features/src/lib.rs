use geo::{Contains, BoundingRect};
use std::collections;

#[derive(Clone, Debug)]
pub struct Feature {
    pub geometry: geo::Geometry<f64>,
    pub properties: collections::HashMap<String, String>,
    pub bounding_rect: geo::Rect<f64>,
}

impl Feature {
    pub fn from_geometry(geometry: geo::Geometry<f64>) -> Result<Self, BoundingBoxError> {
        let bounding_rect = geometry
            .bounding_rect()
            .ok_or(BoundingBoxError)?;

        Ok(Feature {
            geometry,
            properties: collections::HashMap::new(),
            bounding_rect,
        })
    }
}

impl Contains<geo::Coordinate<f64>> for Feature {
    fn contains(&self, coord: &geo::Coordinate<f64>) -> bool {
        self.bounding_rect.contains(coord) && self.geometry.contains(coord)
    }
}

#[derive(Clone, Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bounding_rect: geo::Rect<f64>,
}

#[derive(Debug)]
pub struct BoundingBoxError;

impl FeatureCollection {
    pub fn from_geometry(geometry: geo::Geometry<f64>) -> Result<Self, BoundingBoxError> {
        let bounding_rect = geometry
            .bounding_rect()
            .ok_or(BoundingBoxError)?;
        Ok(FeatureCollection {
            features: vec![Feature::from_geometry(geometry)?],
            bounding_rect,
        })
    }

    pub fn to_geometry_collection(&self) -> geo::GeometryCollection<f64> {
        geo::GeometryCollection(
            self.features
                .iter()
                .map(|f| f.geometry.clone())
                .collect::<Vec<_>>(),
        )
    }

    pub fn bounding_rect(&self) -> Result<geo::Rect<f64>, BoundingBoxError> {
        // TODO: audit performance
        self.to_geometry_collection().bounding_rect().ok_or(BoundingBoxError)
    }
}
