#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;
use geo::bounding_rect::BoundingRect;
use geo::contains::Contains;
use std::{error, sync};

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
            .iter()
            .position(|entry| entry.id == layer_id)
    }

    pub fn get(&self, layer_id: rgis_layer_id::LayerId) -> Option<&Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get(index)
    }

    pub fn get_with_z_index(&self, layer_id: rgis_layer_id::LayerId) -> Option<(&Layer, usize)> {
        let index = self.get_index(layer_id)?;
        self.data.get(index).map(|layer| (layer, index))
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
        rgis_layer_id::LayerId::new()
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
            crs: unassigned_layer.crs,
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
    pub crs: String,
}

impl UnassignedLayer {
    pub fn from_geometry(
        geometry: geo::Geometry<f64>,
        name: String,
        metadata: Option<Metadata>,
        source_crs: &str,
        target_crs: &str,
    ) -> Result<Self, Box<dyn error::Error + Send + Sync>> {
        let unprojected_geometry = geometry;

        let mut projected_geometry = unprojected_geometry.clone();

        let tl = time_logger::start!("Reprojecting");
        #[cfg(target_arch = "wasm32")]
        {
            geo_proj_js::transform(&mut projected_geometry, source_crs, target_crs)?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use geo::transform::Transform;
            projected_geometry.transform_crs_to_crs(source_crs, target_crs)?;
        }
        tl.finish();

        let projected_bounding_rect = projected_geometry
            .bounding_rect()
            .ok_or("Could not generate bounding rectangle for the geometry")?;

        Ok(UnassignedLayer {
            unprojected_geometry,
            projected_geometry,
            projected_bounding_rect,
            color: colorous_color_to_bevy_color(next_colorous_color()),
            metadata: metadata.unwrap_or_else(serde_json::Map::new),
            crs: source_crs.to_string(),
            name,
            visible: true,
        })
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
    pub crs: String,
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

pub struct Plugin;

fn handle_toggle_layer_visibility_events(
    mut toggle_layer_visibility_event_reader: bevy::app::EventReader<
        rgis_events::ToggleLayerVisibilityEvent,
    >,
    mut layer_became_visible_event_writer: bevy::app::EventWriter<
        rgis_events::LayerBecameVisibleEvent,
    >,
    mut layer_became_hidden_event_writer: bevy::app::EventWriter<
        rgis_events::LayerBecameHiddenEvent,
    >,
    mut layers: ResMut<Layers>,
) {
    for event in toggle_layer_visibility_event_reader.iter() {
        let layer = match layers.get_mut(event.0) {
            Some(l) => l,
            None => {
                bevy::log::warn!("Could not find layer");
                continue;
            }
        };
        layer.visible = !layer.visible;
        if layer.visible {
            layer_became_visible_event_writer.send(rgis_events::LayerBecameVisibleEvent(event.0));
        } else {
            layer_became_hidden_event_writer.send(rgis_events::LayerBecameHiddenEvent(event.0));
        }
    }
}

fn handle_update_color_events(
    mut update_events: bevy::app::EventReader<rgis_events::UpdateLayerColor>,
    mut updated_events: bevy::app::EventWriter<rgis_events::LayerColorUpdated>,
    mut layers: ResMut<Layers>,
) {
    for event in update_events.iter() {
        let layer = match layers.get_mut(event.0) {
            Some(l) => l,
            None => {
                bevy::log::warn!("Could not find layer");
                continue;
            }
        };
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

fn handle_move_layer_events(
    mut move_layer_event_reader: bevy::app::EventReader<rgis_events::MoveLayerEvent>,
    mut layer_z_index_updated_event_writer: bevy::app::EventWriter<rgis_events::LayerZIndexUpdated>,
    mut layers: ResMut<Layers>,
) {
    for event in move_layer_event_reader.iter() {
        let (_, old_z_index) = match layers.get_with_z_index(event.0) {
            Some(result) => result,
            None => {
                bevy::log::warn!("Could not find layer");
                continue;
            }
        };

        let new_z_index = match event.1 {
            rgis_events::MoveDirection::Up => {
                old_z_index + 1
            },
            rgis_events::MoveDirection::Down => {
                old_z_index - 1
            },
        };

        let other_layer_id = match layers.data.get(new_z_index) {
            Some(layer) => layer.id,
            None => {
                bevy::log::warn!("Could not find layer");
                continue;
            }
        };

        layers.data.swap(old_z_index, new_z_index);

        layer_z_index_updated_event_writer.send(rgis_events::LayerZIndexUpdated(event.0));
        layer_z_index_updated_event_writer.send(rgis_events::LayerZIndexUpdated(other_layer_id));
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Layers::new())
            .add_system(handle_toggle_layer_visibility_events)
            .add_system(handle_update_color_events)
            .add_system(handle_move_layer_events)
            .add_system(handle_delete_layer_events);
    }
}
