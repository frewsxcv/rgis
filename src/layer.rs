use std::sync;

pub struct Layer {
    pub geometry: geo::Geometry<f64>,
    pub bounding_rect: geo::Rect<f64>,
}

lazy_static::lazy_static! {
    pub static ref LAYERS: sync::RwLock<Vec<Layer>> = {
        sync::RwLock::new(vec![])
    };
}
