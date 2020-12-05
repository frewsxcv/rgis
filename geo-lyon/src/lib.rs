use bevy_prototype_lyon::prelude::*;

pub trait ToPath {
    fn to_path(&self) -> Path;
}

impl ToPath for geo_types::Polygon<f64> {
    fn to_path(&self) -> Path {
        // TODO: interiors
        let mut path_builder = PathBuilder::new();

        let mut exterior_iter = self.exterior().0.iter();

        let first = exterior_iter.next().unwrap();
        path_builder.move_to(point(first.x as f32, first.y as f32));

        for coord in exterior_iter {
            path_builder.line_to(point(coord.x as f32, coord.y as f32))
        }

        path_builder.close();

        path_builder.build()
    }
}

impl ToPath for geo_types::MultiPolygon<f64> {
    fn to_path(&self) -> Path {
        // TODO: interiors
        let mut path_builder = PathBuilder::new();

        for polygon in &self.0 {
            let mut exterior_iter = polygon.exterior().0.iter();

            let first = exterior_iter.next().unwrap();
            path_builder.move_to(point(first.x as f32, first.y as f32));

            for coord in exterior_iter {
                path_builder.line_to(point(coord.x as f32, coord.y as f32))
            }

            path_builder.close();
        }

        path_builder.build()
    }
}
