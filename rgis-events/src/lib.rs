use bevy::prelude::*;
use geo_projected::{ProjectedCoord, UnprojectedScalar};

// Magic number used to normalize the host's scroll value.
const ZOOM_FACTOR: f32 = 500.;

#[derive(Debug, Event)]
pub struct LayerCreatedEvent(pub rgis_primitives::LayerId);

#[derive(Debug, Event)]
pub struct ShowManageLayerWindowEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct ToggleLayerVisibilityEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct CenterCameraEvent(pub rgis_primitives::LayerId);

impl From<rgis_primitives::LayerId> for CenterCameraEvent {
    #[inline]
    fn from(layer_id: rgis_primitives::LayerId) -> Self {
        CenterCameraEvent(layer_id)
    }
}

#[derive(Clone, Copy, Event, Debug)]
pub struct FeatureSelectedEvent(pub rgis_primitives::LayerId, pub geo_features::FeatureId);

#[derive(Event, Debug)]
pub struct LayerBecameHiddenEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct LayerBecameVisibleEvent(pub rgis_primitives::LayerId);

/// Change the `Layer`'s color
#[derive(Event)]
pub enum UpdateLayerColorEvent {
    Fill(rgis_primitives::LayerId, bevy::prelude::Color),
    Stroke(rgis_primitives::LayerId, bevy::prelude::Color),
}
/// After a `Layer`'s color is changed
#[derive(Clone, Copy, Event)]
pub enum LayerColorUpdatedEvent {
    Fill(rgis_primitives::LayerId),
    Stroke(rgis_primitives::LayerId),
}

#[derive(Event)]
pub struct DeleteLayerEvent(pub rgis_primitives::LayerId);

#[derive(Event)]
pub struct DespawnMeshesEvent(pub rgis_primitives::LayerId);

#[derive(Event)]
pub struct MeshesSpawnedEvent(pub rgis_primitives::LayerId);

impl From<rgis_primitives::LayerId> for MeshesSpawnedEvent {
    #[inline]
    fn from(layer_id: rgis_primitives::LayerId) -> Self {
        MeshesSpawnedEvent(layer_id)
    }
}

pub enum MoveDirection {
    Up,
    Down,
}

#[derive(Event)]
pub struct MoveLayerEvent(pub rgis_primitives::LayerId, pub MoveDirection);

#[derive(Event)]
pub struct LayerZIndexUpdatedEvent(pub rgis_primitives::LayerId);

#[derive(Event)]
pub struct MapClickedEvent(pub ProjectedCoord);

#[derive(Event)]
pub struct FeaturesDeselectedEvent;

#[derive(Default, Event)]
pub struct OpenChangeCrsWindow;

#[derive(Event, Debug)]
pub enum LoadFileEvent {
    FromNetwork {
        name: String,
        url: String,
        source_crs: rgis_primitives::Crs,
    },
    FromBytes {
        file_name: String,
        file_format: geo_file_loader::FileFormat,
        bytes: bytes::Bytes,
        source_crs: rgis_primitives::Crs,
    },
}

pub struct Plugin;

#[derive(Event, Debug)]
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

#[derive(Event, Debug)]
pub struct ZoomCameraEvent {
    /// * `amount ∈ (1, ∞)` → zoom in
    /// * `amount ∈ [1]` → no change
    /// * `amount ∈ (0, 1)` → zoom out
    pub amount: f32,
    pub coord: ProjectedCoord,
}

impl ZoomCameraEvent {
    #[inline]
    pub fn new(amount: f32, coord: ProjectedCoord) -> Self {
        ZoomCameraEvent {
            // Don't let amount be negative, so add `max`
            amount: (1. + amount / ZOOM_FACTOR).max(0.),
            coord,
        }
    }
}

#[derive(Event)]
pub struct ChangeCrsEvent {
    pub old: rgis_primitives::Crs,
    pub new: rgis_primitives::Crs,
}

#[derive(Event)]
pub struct CrsChangedEvent {
    pub old: rgis_primitives::Crs,
    pub new: rgis_primitives::Crs,
}

#[derive(Event)]
pub struct RenderMessageEvent(pub String);

#[derive(Event)]
pub struct RenderFeaturePropertiesEvent {
    pub layer_id: rgis_primitives::LayerId,
    pub properties: geo_features::Properties,
}

#[derive(Event)]
pub struct CreateLayerEvent {
    pub feature_collection: geo_features::FeatureCollection<UnprojectedScalar>,
    pub name: String,
    pub source_crs: rgis_primitives::Crs,
}

#[derive(Event)]
pub struct LayerReprojectedEvent(pub rgis_primitives::LayerId);

#[derive(Default, Event)]
pub struct ShowAddLayerWindow;

#[derive(Default, Event)]
pub struct HideAddLayerWindow;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadFileEvent>()
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
            .add_event::<FeaturesDeselectedEvent>()
            .add_event::<ShowManageLayerWindowEvent>();
    }
}
