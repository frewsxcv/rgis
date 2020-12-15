use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;
use std::sync;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LayerId(i64);

#[derive(Clone, Debug)]
pub struct Layers {
    pub data: Vec<Layer>,
    pub projected_bounding_rect: Option<geo_srs::RectWithSrs<f64>>,
    // ID of the currently selected Layer
    pub selected_layer_id: Option<LayerId>,
}

pub type Color = (u8, u8, u8);

impl Layers {
    pub fn new() -> Layers {
        Layers {
            data: vec![],
            projected_bounding_rect: None,
            selected_layer_id: None,
        }
    }

    // coord is assumed to be projected
    pub fn containing_coord(&self, coord: geo_srs::CoordWithSrs<f64>) -> Vec<Layer> {
        let projected_bounding_rect = match &self.projected_bounding_rect {
            Some(b) => b,
            None => return vec![],
        };

        if !projected_bounding_rect.contains(&coord) {
            return vec![];
        }

        self.data
            .iter()
            .filter(|layer| layer.contains_coord(&coord))
            .cloned()
            .collect()
    }

    // Returns whether the selected layer changed
    pub fn set_selected_layer_from_mouse_press(
        &mut self,
        coord: geo_srs::CoordWithSrs<f64>,
    ) -> bool {
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

    pub fn get(&self, layer_id: LayerId) -> Option<&Layer> {
        self.data
            .binary_search_by_key(&layer_id, |layer| layer.id)
            .ok()
            .and_then(|layer_index| self.data.get(layer_index))
    }

    #[allow(unused)]
    pub fn selected_layer(&self) -> Option<&Layer> {
        self.selected_layer_id
            .and_then(|layer_id| self.get(layer_id))
    }

    fn next_layer_id(&self) -> LayerId {
        LayerId(self.data.last().map(|layer| layer.id.0 + 1).unwrap_or(1))
    }

    pub fn add(
        &mut self,
        geometry: geo::Geometry<f64>,
        metadata: Option<Metadata>,
        source_projection: &str,
        target_projection: &str,
    ) -> LayerId {
        let layer_id = self.next_layer_id();
        let layer = Layer::from_geometry(
            geometry,
            layer_id,
            metadata,
            source_projection,
            target_projection,
        );
        self.projected_bounding_rect = Some(if let Some(r) = self.projected_bounding_rect.clone() {
            r.merge(&layer.projected_bounding_rect)
        } else {
            layer.projected_bounding_rect.clone()
        });
        self.data.push(layer);
        layer_id
    }
}

pub type Metadata = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct Layer {
    pub unprojected_geometry: geo_srs::GeometryWithSrs<f64>,
    pub unprojected_bounding_rect: geo_srs::RectWithSrs<f64>,
    pub projected_geometry: geo_srs::GeometryWithSrs<f64>,
    pub projected_bounding_rect: geo_srs::RectWithSrs<f64>,
    pub color: Color,
    pub metadata: Metadata,
    pub id: LayerId,
}

impl Layer {
    pub fn contains_coord(&self, coord: &geo_srs::CoordWithSrs<f64>) -> bool {
        self.projected_bounding_rect.contains(&coord)
            && self.projected_geometry.geometry.contains(&coord.coord)
    }

    pub fn from_geometry(
        geometry: geo::Geometry<f64>,
        id: LayerId,
        metadata: Option<Metadata>,
        source_projection: &str,
        target_projection: &str,
    ) -> Self {
        let unprojected_geometry = geo_srs::GeometryWithSrs {
            geometry,
            srs: source_projection.into(),
        };
        let unprojected_bounding_rect = geo_srs::RectWithSrs {
            rect: unprojected_geometry
                .geometry
                .bounding_rect()
                .expect("Could not determine bounding rect of geometry"),
            srs: unprojected_geometry.srs.to_owned(),
        };

        let mut projected_geometry = unprojected_geometry.clone();

        let tl = time_logger::start("Reprojecting");
        projected_geometry.reproject(target_projection);
        tl.finish();

        let projected_bounding_rect = geo_srs::RectWithSrs {
            rect: projected_geometry
                .geometry
                .bounding_rect()
                .expect("Could not determine bounding rect of geometry"),
            srs: projected_geometry.srs.to_string(),
        };

        Layer {
            unprojected_geometry,
            unprojected_bounding_rect,
            projected_geometry,
            projected_bounding_rect,
            color: next_colorous_color().as_tuple(),
            metadata: metadata.unwrap_or_else(serde_json::Map::new),
            id,
        }
    }
}

const COLORS: [colorous::Color; 10] = colorous::CATEGORY10;

fn next_colorous_color() -> colorous::Color {
    COLORS[next_color_index()]
}

fn next_color_index() -> usize {
    static COUNTER: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);
    COUNTER.fetch_add(1, sync::atomic::Ordering::Relaxed) % COLORS.len()
}
