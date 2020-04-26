use crate::renderable::next_color;
use geo::bounding_rect::BoundingRect;
use pathfinder_canvas::ColorU;
use std::sync;

lazy_static::lazy_static! {
    pub static ref LAYERS: Layers = Layers {
        data: sync::RwLock::new(vec![]),
        bounding_rect: sync::RwLock::new(None),
    };
}

pub struct Layers {
    pub data: sync::RwLock<Vec<Layer>>,
    pub bounding_rect: sync::RwLock<Option<geo::Rect<f64>>>,
}

impl Layers {
    pub fn add(&self, geometry: geo::Geometry<f64>) {
        let layer = Layer::from_geometry(geometry);
        let mut guard = self.bounding_rect.write().unwrap();
        if let Some(r) = *guard {
            *guard = Some(bbox_merge(r, layer.bounding_rect));
        } else {
            *guard = Some(layer.bounding_rect);
        }
        self.data.write().unwrap().push(layer);
    }
}

fn geometry_bounding_rect(geometry: &geo::Geometry<f64>) -> geo::Rect<f64> {
    match geometry {
        geo::Geometry::LineString(line_string) => line_string.bounding_rect().unwrap(),
        geo::Geometry::Polygon(polygon) => polygon.bounding_rect().unwrap(),
        geo::Geometry::MultiLineString(multi_line_string) => {
            multi_line_string.bounding_rect().unwrap()
        }
        geo::Geometry::MultiPolygon(multi_polygon) => multi_polygon.bounding_rect().unwrap(),
        _ => unimplemented!(),
    }
}

fn bbox_merge(a: geo::Rect<f64>, b: geo::Rect<f64>) -> geo::Rect<f64> {
    geo::Rect::new(
        geo::Coordinate {
            x: a.min().x.min(b.min().x),
            y: a.min().y.min(b.min().y),
        },
        geo::Coordinate {
            x: a.max().x.max(b.max().x),
            y: a.max().y.max(b.max().y),
        },
    )
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
