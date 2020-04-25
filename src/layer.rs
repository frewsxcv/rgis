use pathfinder_canvas::ColorU;
use crate::renderable::next_color;
use geo::bounding_rect::BoundingRect;
use std::sync;

lazy_static::lazy_static! {
    pub static ref LAYERS: Layers = Layers(
        sync::RwLock::new(vec![])
    );
}

pub struct Layers(pub sync::RwLock<Vec<Layer>>);

impl Layers {
    pub fn add(&self, geometry: geo::Geometry<f64>) {
        self.0.write().unwrap().push(Layer::from_geometry(geometry));
    }
}

fn geometry_bounding_rect(geometry: &geo::Geometry<f64>) -> geo::Rect<f64> {
    match geometry {
        geo::Geometry::LineString(line_string) => line_string.bounding_rect().unwrap(),
        geo::Geometry::Polygon(polygon) => polygon.bounding_rect().unwrap(),
        geo::Geometry::MultiLineString(multi_line_string) => multi_line_string.bounding_rect().unwrap(),
        geo::Geometry::MultiPolygon(multi_polygon) => multi_polygon.bounding_rect().unwrap(),
        _ => unimplemented!(),
    }
}

pub struct Layer {
    pub geometry: geo::Geometry<f64>,
    pub bounding_rect: geo::Rect<f64>,
    pub color: ColorU,
}

impl Layer {
    pub fn from_geometry(geometry: geo::Geometry<f64>) -> Self {
        Layer {
            bounding_rect: geometry_bounding_rect(&geometry),
            geometry: geometry,
            color: next_color(),
        }
    }
}
