use bevy_prototype_lyon::prelude::*;

pub trait ToPath {
    fn to_path(&self) -> Path;
}

impl ToPath for geo_types::Polygon<f64> {
    fn to_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();

        polygon_path_builder(self, &mut path_builder);

        path_builder.build()
    }
}

impl ToPath for geo_types::MultiPolygon<f64> {
    fn to_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();

        multi_polygon_path_builder(self, &mut path_builder);

        path_builder.build()
    }
}

impl ToPath for geo_types::GeometryCollection<f64> {
    fn to_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();

        geometry_collection_path_builder(self, &mut path_builder);

        path_builder.build()
    }
}

fn geometry_collection_path_builder(geometry_collection: &geo_types::GeometryCollection<f64>, path_builder: &mut PathBuilder) {
    for geometry_collection in &geometry_collection.0 {
        geometry_path_builder(geometry_collection, path_builder)
    }
}

fn geometry_path_builder(geometry: &geo_types::Geometry<f64>, path_builder: &mut PathBuilder) {
    match geometry {
        geo_types::Geometry::Polygon(g) => polygon_path_builder(g, path_builder),
        geo_types::Geometry::MultiPolygon(g) => multi_polygon_path_builder(g, path_builder),
        geo_types::Geometry::GeometryCollection(g) => geometry_collection_path_builder(g, path_builder),
        _ => {
            log::error!("Encountered a geometry type we don’t know how to render");
            return
        }
    }
}

fn multi_polygon_path_builder(multi_polygon: &geo_types::MultiPolygon<f64>, path_builder: &mut PathBuilder) {
    for polygon in &multi_polygon.0 {
        polygon_path_builder(polygon, path_builder);
    }
}

fn polygon_path_builder(polygon: &geo_types::Polygon<f64>, path_builder: &mut PathBuilder) {
    ring_path_builder(polygon.exterior(), path_builder);

    for interior in polygon.interiors() {
        ring_path_builder(interior, path_builder);
    }
}

fn ring_path_builder(
    ring_line_string: &geo_types::LineString<f64>,
    path_builder: &mut PathBuilder,
) {
    coords_path_builder(ring_line_string.0.iter().copied(), path_builder);
    path_builder.close();
}

fn coords_path_builder(
    mut iter: impl Iterator<Item = geo_types::Coordinate<f64>>,
    path_builder: &mut PathBuilder,
) {
    let first = match iter.next() {
        Some(coord) => coord,
        None => return,
    };
    path_builder.move_to(point(first.x as f32, first.y as f32));

    for coord in iter {
        path_builder.line_to(point(coord.x as f32, coord.y as f32))
    }
}
