use crate::{Operation, OperationEntry, Outcome};
use geo::OutlierDetection;
use std::{error, mem};

impl OperationEntry for Outliers {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::POINT.bits() | geo_geom_type::GeomType::MULTI_POINT.bits(),
    );
    const NAME: &'static str = "Detect outliers";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<Outliers>::default()
    }
}

#[derive(Default)]
pub struct Outliers {
    points: Vec<geo::Point>,
}

impl Operation for Outliers {
    fn visit_point(&mut self, point: geo::Point) {
        self.points.push(point);
    }

    fn visit_multi_point(&mut self, multi_point: geo::MultiPoint) {
        self.points.extend(multi_point.0.into_iter());
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>> {
        let mut non_outliers = vec![];
        let points = mem::take(&mut self.points);

        let multi_point = geo::MultiPoint(points);

        for (outlier_score, coord) in multi_point.outliers(15).iter().zip(multi_point.0.iter()) {
            if *outlier_score < 2. {
                non_outliers.push(*coord);
            }
        }

        let new_multi_point = geo::MultiPoint::new(non_outliers);

        Ok(Outcome::FeatureCollection(geo_projected::Unprojected::new(
            geo_features::FeatureCollection::from_geometry(new_multi_point.into())?,
        )))
    }
}
