#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

// Magic number used to normalize the host's scroll value.
const ZOOM_FACTOR: f32 = 500.;

#[derive(Debug)]
pub struct LayerCreatedEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct ToggleLayerVisibilityEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct CenterCameraEvent(pub rgis_layer_id::LayerId);

impl From<rgis_layer_id::LayerId> for CenterCameraEvent {
    #[inline]
    fn from(layer_id: rgis_layer_id::LayerId) -> Self {
        CenterCameraEvent(layer_id)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FeatureSelectedEvent(pub rgis_layer_id::LayerId, pub geo_features::FeatureId);

#[derive(Debug)]
pub struct LayerBecameHiddenEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct LayerBecameVisibleEvent(pub rgis_layer_id::LayerId);

/// Change the `Layer`'s color
pub enum UpdateLayerColorEvent {
    Fill(rgis_layer_id::LayerId, bevy::prelude::Color),
    Border(rgis_layer_id::LayerId, bevy::prelude::Color),
}
/// After a `Layer`'s color is changed
#[derive(Clone, Copy)]
pub enum LayerColorUpdatedEvent {
    Fill(rgis_layer_id::LayerId),
    Border(rgis_layer_id::LayerId),
}

pub struct DeleteLayerEvent(pub rgis_layer_id::LayerId);

pub struct DespawnMeshesEvent(pub rgis_layer_id::LayerId);

pub struct MeshesSpawnedEvent(pub rgis_layer_id::LayerId);

impl From<rgis_layer_id::LayerId> for MeshesSpawnedEvent {
    #[inline]
    fn from(layer_id: rgis_layer_id::LayerId) -> Self {
        MeshesSpawnedEvent(layer_id)
    }
}

pub enum MoveDirection {
    Up,
    Down,
}
pub struct MoveLayerEvent(pub rgis_layer_id::LayerId, pub MoveDirection);

pub struct LayerZIndexUpdatedEvent(pub rgis_layer_id::LayerId);

pub struct MapClickedEvent(pub geo_projected::Projected<geo::Coord>);

pub struct FeaturesDeselectedEvent;

#[derive(Default)]
pub struct OpenChangeCrsWindow;

#[derive(Debug)]
pub enum LoadFileEvent<F: geo_file_loader::FileLoader> {
    FromNetwork {
        name: String,
        url: String,
        crs: String,
    },
    FromBytes {
        file_name: String,
        file_loader: F,
        crs: String,
    },
}

pub struct Plugin;

#[derive(Debug)]
pub struct PanCameraEvent {
    // X offset for camera position. Positive is right, negative is left.
    pub x: f32,
    // Y offset for camera position. Positive is up, negative is down.
    pub y: f32,
}

impl PanCameraEvent {
    #[inline]
    pub fn up(amount: f32) -> Self {
        PanCameraEvent { x: 0., y: amount }
    }

    #[inline]
    pub fn right(amount: f32) -> Self {
        PanCameraEvent { x: amount, y: 0. }
    }

    #[inline]
    pub fn down(amount: f32) -> Self {
        PanCameraEvent { x: 0., y: -amount }
    }

    #[inline]
    pub fn left(amount: f32) -> Self {
        PanCameraEvent { x: -amount, y: 0. }
    }
}

#[derive(Debug)]
pub struct ZoomCameraEvent {
    /// * `amount ∈ (1, ∞)` → zoom in
    /// * `amount ∈ [1]` → no change
    /// * `amount ∈ (0, 1)` → zoom out
    pub amount: f32,
    pub coord: geo_projected::Projected<geo::Coord>,
}

impl ZoomCameraEvent {
    #[inline]
    pub fn new(amount: f32, coord: geo_projected::Projected<geo::Coord>) -> Self {
        ZoomCameraEvent {
            // Don't let amount be negative, so add `max`
            amount: (1. + amount / ZOOM_FACTOR).max(0.),
            coord,
        }
    }
}

pub struct ChangeCrsEvent {
    pub old_crs: String,
    pub new_crs: String,
}

pub struct CrsChangedEvent {
    pub old_crs: String,
    pub new_crs: String,
}

pub struct RenderMessageEvent(pub String);

pub struct RenderFeaturePropertiesEvent(pub geo_features::Properties);

pub struct CreateLayerEvent {
    pub feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    pub name: String,
    pub source_crs: String,
}

pub struct LayerReprojectedEvent(pub rgis_layer_id::LayerId);

#[derive(Default)]
pub struct ShowAddLayerWindow;

#[derive(Default)]
pub struct HideAddLayerWindow;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadFileEvent<geo_file_loader::GeoJsonSource>>()
            .add_event::<LoadFileEvent<geo_file_loader::WktSource>>()
            .add_event::<LoadFileEvent<geo_file_loader::ShapefileSource>>()
            .add_event::<CreateLayerEvent>()
            .add_event::<LayerCreatedEvent>()
            .add_event::<ToggleLayerVisibilityEvent>()
            .add_event::<LayerBecameHiddenEvent>()
            .add_event::<LayerBecameVisibleEvent>()
            .add_event::<PanCameraEvent>()
            .add_event::<ZoomCameraEvent>()
            .add_event::<CenterCameraEvent>()
            .add_event::<LayerColorUpdatedEvent>()
            .add_event::<UpdateLayerColorEvent>()
            .add_event::<MoveLayerEvent>()
            .add_event::<LayerZIndexUpdatedEvent>()
            .add_event::<DeleteLayerEvent>()
            .add_event::<MeshesSpawnedEvent>()
            .add_event::<ChangeCrsEvent>()
            .add_event::<CrsChangedEvent>()
            .add_event::<MapClickedEvent>()
            .add_event::<RenderMessageEvent>()
            .add_event::<RenderFeaturePropertiesEvent>()
            .add_event::<OpenChangeCrsWindow>()
            .add_event::<ShowAddLayerWindow>()
            .add_event::<HideAddLayerWindow>()
            .add_event::<LayerReprojectedEvent>()
            .add_event::<DespawnMeshesEvent>()
            .add_event::<FeatureSelectedEvent>()
            .add_event::<FeaturesDeselectedEvent>();
    }
}
