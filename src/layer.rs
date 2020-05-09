use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;
use pathfinder_canvas::ColorU;

#[derive(Clone, Debug)]
pub struct Layers {
    pub data: Vec<Layer>,
    pub bounding_rect: Option<geo::Rect<f64>>,
    // ID of the currently selected Layer
    pub selected_layer_id: Option<i64>,
}

impl Layers {
    pub fn new() -> Layers {
        Layers {
            data: vec![],
            bounding_rect: None,
            selected_layer_id: None,
        }
    }

    pub fn containing_coord(&self, coord: geo::Coordinate<f64>) -> Vec<Layer> {
        let bounding_rect = match self.bounding_rect {
            Some(b) => b,
            None => return vec![],
        };

        if !bounding_rect.contains(&coord) {
            return vec![];
        }

        self.data
            .iter()
            .filter(|layer| layer.contains_coord(coord))
            .cloned()
            .collect()
    }

    // Returns whether the selected layer changed
    pub fn set_selected_layer_from_mouse_press(&mut self, coord: geo::Coordinate<f64>) -> bool {
        let intersecting = self.containing_coord(coord);
        if !intersecting.is_empty() {
            log::info!("A geometry was clicked: {:?}", intersecting[0].metadata);
        }
        if intersecting.len() > 1 {
            log::warn!("Multiple layers clicked. Choosing one randomly.");
        }
        let prev_selected_layer_id = self.selected_layer_id;

        self.selected_layer_id = intersecting.get(0).map(|layer| layer.id);

        prev_selected_layer_id != self.selected_layer_id
    }

    #[allow(unused)]
    pub fn selected_layer(&self) -> Option<&Layer> {
        self.selected_layer_id
            .and_then(|layer_id| {
                self.data
                    .binary_search_by_key(&layer_id, |layer| layer.id)
                    .ok()
            })
            .and_then(|layer_index| self.data.get(layer_index))
    }

    fn next_layer_id(&self) -> Id {
        self.data.last().map(|layer| layer.id + 1).unwrap_or(1)
    }

    pub fn add(&mut self, geometry: geo::Geometry<f64>, metadata: Option<Metadata>) {
        let layer = Layer::from_geometry(geometry, self.next_layer_id(), metadata);
        self.bounding_rect = Some(if let Some(r) = self.bounding_rect {
            bbox_merge(r, layer.bounding_rect)
        } else {
            layer.bounding_rect
        });
        self.data.push(layer);
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

pub type Metadata = serde_json::Map<String, serde_json::Value>;
pub type Id = i64;

#[derive(Clone, Debug)]
pub struct Layer {
    pub geometry: geo::Geometry<f64>,
    pub bounding_rect: geo::Rect<f64>,
    pub color: ColorU,
    pub metadata: Metadata,
    pub id: Id,
}

impl Layer {
    pub fn contains_coord(&self, coord: geo::Coordinate<f64>) -> bool {
        self.bounding_rect.contains(&geo::Point(coord))
            && self.geometry.contains(&geo::Point(coord))
    }

    pub fn from_geometry(
        geometry: geo::Geometry<f64>,
        id: i64,
        metadata: Option<Metadata>,
    ) -> Self {
        let bounding_rect = geometry
            .bounding_rect()
            .expect("Could not determine bounding rect of geometry");

        Layer {
            bounding_rect,
            geometry,
            color: crate::color::next(),
            metadata: metadata.unwrap_or_else(serde_json::Map::new),
            id,
        }
    }
}
