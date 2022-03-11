use bevy::prelude::*;
use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;
use std::sync;

#[derive(Clone, Debug)]
pub struct Layers {
    pub data: Vec<Layer>,
    // ID of the currently selected Layer
    pub selected_layer_id: Option<rgis_layer_id::LayerId>,
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl Layers {
    pub fn new() -> Layers {
        Layers {
            data: vec![],
            selected_layer_id: None,
        }
    }

    // coord is assumed to be projected
    pub fn containing_coord(&self, coord: geo::Coordinate<f64>) -> Vec<Layer> {
        self.data
            .iter()
            .filter(|layer| layer.contains_coord(&coord))
            .cloned()
            .collect()
    }

    // Returns whether the selected layer changed
    pub fn set_selected_layer_from_mouse_press(&mut self, coord: geo::Coordinate<f64>) -> bool {
        let intersecting = self.containing_coord(coord);
        if !intersecting.is_empty() {
            info!("A geometry was clicked: {:?}", intersecting[0].metadata);
        }
        if intersecting.len() > 1 {
            warn!("Multiple layers clicked. Choosing one randomly.");
        }
        let prev_selected_layer_id = self.selected_layer_id;

        self.selected_layer_id = intersecting.get(0).map(|layer| layer.id);

        prev_selected_layer_id != self.selected_layer_id
    }

    fn get_index(&self, layer_id: rgis_layer_id::LayerId) -> Option<usize> {
        self.data
            .binary_search_by_key(&layer_id, |layer| layer.id)
            .ok()
    }

    pub fn get(&self, layer_id: rgis_layer_id::LayerId) -> Option<&Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get(index)
    }

    pub fn get_mut(&mut self, layer_id: rgis_layer_id::LayerId) -> Option<&mut Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get_mut(index)
    }

    pub fn remove(&mut self, layer_id: rgis_layer_id::LayerId) {
        if let Some(index) = self.get_index(layer_id) {
            self.data.remove(index);
        }
    }

    #[allow(unused)]
    pub fn selected_layer(&self) -> Option<&Layer> {
        self.selected_layer_id
            .and_then(|layer_id| self.get(layer_id))
    }

    fn next_layer_id(&self) -> rgis_layer_id::LayerId {
        rgis_layer_id::LayerId(self.data.last().map(|layer| layer.id.0 + 1).unwrap_or(1))
    }

    pub fn add(&mut self, unassigned_layer: UnassignedLayer) -> rgis_layer_id::LayerId {
        let layer_id = self.next_layer_id();
        let layer = Layer {
            unprojected_geometry: unassigned_layer.unprojected_geometry,
            projected_geometry: unassigned_layer.projected_geometry,
            projected_bounding_rect: unassigned_layer.projected_bounding_rect,
            color: unassigned_layer.color,
            metadata: unassigned_layer.metadata,
            name: unassigned_layer.name,
            visible: unassigned_layer.visible,
            id: layer_id,
        };
        self.data.push(layer);
        layer_id
    }
}

pub type Metadata = serde_json::Map<String, serde_json::Value>;

#[derive(Debug)]
pub struct UnassignedLayer {
    pub unprojected_geometry: geo::Geometry<f64>,
    pub projected_geometry: geo::Geometry<f64>,
    pub projected_bounding_rect: geo::Rect<f64>,
    pub color: Color,
    pub metadata: Metadata,
    pub name: String,
    pub visible: bool,
}

impl UnassignedLayer {
    pub fn from_geometry(
        geometry: geo::Geometry<f64>,
        name: String,
        metadata: Option<Metadata>,
        source_projection: &str,
        target_projection: &str,
    ) -> Self {
        let unprojected_geometry = geometry;

        let mut projected_geometry = unprojected_geometry.clone();

        let tl = time_logger::start!("Reprojecting");
        #[cfg(target_arch = "wasm32")]
        {
            use geo::algorithm::map_coords::MapCoordsInplace;
            use wasm_bindgen::JsCast;
            let proj4 = web_sys::window()
                .unwrap()
                .get("proj4")
                .unwrap()
                .dyn_into::<js_sys::Function>()
                .unwrap();
            let projector = proj4
                .call2(
                    &wasm_bindgen::JsValue::UNDEFINED,
                    &source_projection.into(),
                    &target_projection.into(),
                )
                .unwrap();
            let mut array = js_sys::Array::new_with_length(2);
            let forward = js_sys::Reflect::get(&projector, &"forward".into())
                .unwrap()
                .dyn_into::<js_sys::Function>()
                .unwrap();
            projected_geometry.map_coords_inplace(|(x, y)| {
                array.set(0, wasm_bindgen::JsValue::from_f64(*x));
                array.set(1, wasm_bindgen::JsValue::from_f64(*y));
                let result = forward
                    .call1(&wasm_bindgen::JsValue::UNDEFINED, &array)
                    .unwrap()
                    .dyn_into::<js_sys::Array>()
                    .unwrap();
                (
                    result.get(0).as_f64().unwrap(),
                    result.get(1).as_f64().unwrap(),
                )
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use geo::transform::Transform;
            projected_geometry
                .transform_crs_to_crs(source_projection, target_projection)
                .unwrap();
        }
        tl.finish();

        let projected_bounding_rect = projected_geometry
            .bounding_rect()
            .expect("Could not determine bounding rect of geometry");

        UnassignedLayer {
            unprojected_geometry,
            projected_geometry,
            projected_bounding_rect,
            color: colorous_color_to_bevy_color(next_colorous_color()),
            metadata: metadata.unwrap_or_else(serde_json::Map::new),
            name,
            visible: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub unprojected_geometry: geo::Geometry<f64>,
    pub projected_geometry: geo::Geometry<f64>,
    pub projected_bounding_rect: geo::Rect<f64>,
    pub color: Color,
    pub metadata: Metadata,
    pub id: rgis_layer_id::LayerId,
    pub name: String,
    pub visible: bool,
}

impl Layer {
    pub fn contains_coord(&self, coord: &geo::Coordinate<f64>) -> bool {
        self.projected_bounding_rect.contains(coord) && self.projected_geometry.contains(coord)
    }
}

fn colorous_color_to_bevy_color(colorous_color: colorous::Color) -> Color {
    Color::rgb_u8(colorous_color.r, colorous_color.g, colorous_color.b)
}

const COLORS: [colorous::Color; 10] = colorous::CATEGORY10;

fn next_colorous_color() -> colorous::Color {
    COLORS[next_color_index()]
}

fn next_color_index() -> usize {
    static COUNTER: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);
    COUNTER.fetch_add(1, sync::atomic::Ordering::Relaxed) % COLORS.len()
}

pub struct RgisLayersPlugin;

fn handle_toggle_layer_visibility_events(
    mut toggle_layer_visibility_event_reader: bevy::app::EventReader<
        rgis_events::ToggleLayerVisibilityEvent,
    >,
    mut layers: ResMut<Layers>,
) {
    for event in toggle_layer_visibility_event_reader.iter() {
        let layer = layers.get_mut(event.0).unwrap();
        layer.visible = !layer.visible;
    }
}

fn handle_update_color_events(
    mut update_events: bevy::app::EventReader<rgis_events::UpdateLayerColor>,
    mut updated_events: bevy::app::EventWriter<rgis_events::LayerColorUpdated>,
    mut layers: ResMut<Layers>,
) {
    for event in update_events.iter() {
        let layer = layers.get_mut(event.0).unwrap();
        layer.color = event.1;
        updated_events.send(rgis_events::LayerColorUpdated(event.0));
    }
}

fn handle_delete_layer_events(
    mut delete_layer_event_reader: bevy::app::EventReader<rgis_events::DeleteLayer>,
    mut layer_deleted_event_writer: bevy::app::EventWriter<rgis_events::LayerDeleted>,
    mut layers: ResMut<Layers>,
) {
    for event in delete_layer_event_reader.iter() {
        layers.remove(event.0);
        layer_deleted_event_writer.send(rgis_events::LayerDeleted(event.0));
    }
}

impl Plugin for RgisLayersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Layers::new())
            .add_system(handle_toggle_layer_visibility_events)
            .add_system(handle_update_color_events)
            .add_system(handle_delete_layer_events);
    }
}
