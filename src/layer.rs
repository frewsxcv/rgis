use std::sync;

pub struct Layer {
    pub geometry: geo::Geometry<f64>,
    pub bounding_rect: geo::Rect<f64>,
}

pub type Layers = sync::Arc<sync::RwLock<Vec<Layer>>>;
