use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;
use pathfinder_canvas::ColorU;

#[derive(Clone, Debug)]
pub struct Layers {
    pub data: Vec<Layer>,
    pub projected_bounding_rect: Option<geo_srs::RectWithSrs<f64>>,
    // ID of the currently selected Layer
    pub selected_layer_id: Option<i64>,
}

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
        let projected_bounding_rect = match self.projected_bounding_rect {
            Some(b) => b,
            None => return vec![],
        };

        if !projected_bounding_rect.contains(&coord) {
            return vec![];
        }

        self.data
            .iter()
            .filter(|layer| layer.contains_coord(coord))
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
        self.projected_bounding_rect = Some(if let Some(r) = self.projected_bounding_rect {
            r.merge(layer.projected_bounding_rect)
        } else {
            layer.projected_bounding_rect
        });
        self.data.push(layer);
    }
}

pub type Metadata = serde_json::Map<String, serde_json::Value>;
pub type Id = i64;

#[derive(Clone, Debug)]
pub struct Layer {
    pub unprojected_geometry: geo_srs::GeometryWithSrs<f64>,
    pub unprojected_bounding_rect: geo_srs::RectWithSrs<f64>,
    pub projected_geometry: geo_srs::GeometryWithSrs<f64>,
    pub projected_bounding_rect: geo_srs::RectWithSrs<f64>,
    pub color: ColorU,
    pub metadata: Metadata,
    pub id: Id,
}

impl Layer {
    pub fn contains_coord(&self, coord: geo_srs::CoordWithSrs<f64>) -> bool {
        self.projected_bounding_rect.contains(&coord)
            && self.projected_geometry.geometry.contains(&coord.coord)
    }

    pub fn from_geometry(
        geometry: geo::Geometry<f64>,
        id: i64,
        metadata: Option<Metadata>,
    ) -> Self {
        let unprojected_geometry = geo_srs::GeometryWithSrs {
            geometry,
            srs: crate::SOURCE_PROJECTION,
        };
        let unprojected_bounding_rect = geo_srs::RectWithSrs {
            rect: unprojected_geometry
                .geometry
                .bounding_rect()
                .expect("Could not determine bounding rect of geometry"),
            srs: unprojected_geometry.srs,
        };

        let mut projected_geometry = unprojected_geometry.clone();
        projected_geometry.reproject(crate::TARGET_PROJECTION);
        let projected_bounding_rect = geo_srs::RectWithSrs {
            rect: projected_geometry
                .geometry
                .bounding_rect()
                .expect("Could not determine bounding rect of geometry"),
            srs: projected_geometry.srs,
        };

        Layer {
            unprojected_geometry,
            unprojected_bounding_rect,
            projected_geometry,
            projected_bounding_rect,
            color: crate::color::next(),
            metadata: metadata.unwrap_or_else(serde_json::Map::new),
            id,
        }
    }
}
