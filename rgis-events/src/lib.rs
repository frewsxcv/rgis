use bevy::prelude::*;
use std::path;

const ZOOM_AMOUNT: f32 = 1.15; // Larger number will zoom more

#[derive(Debug)]
pub struct LayerLoadedEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct ToggleLayerVisibilityEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct CenterCameraEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub enum ToggleMaterialEvent {
    Show(rgis_layer_id::LayerId),
    Hide(rgis_layer_id::LayerId),
}

#[derive(Debug)]
pub enum LoadGeoJsonFileEvent {
    FromPath {
        path: path::PathBuf,
        source_srs: String,
        target_srs: String,
    },
    FromBytes {
        file_name: String,
        bytes: Vec<u8>,
        source_srs: String,
        target_srs: String,
    },
}

pub struct RgisEventsPlugin;

#[derive(Debug)]
pub struct PanCameraEvent {
    // X offset for camera position. Positive is right, negative is left.
    pub x: f32,
    // Y offset for camera position. Positive is up, negative is down.
    pub y: f32,
}

impl PanCameraEvent {
    pub fn up(amount: f32) -> Self {
        PanCameraEvent { x: 0., y: amount }
    }

    pub fn right(amount: f32) -> Self {
        PanCameraEvent { x: amount, y: 0. }
    }

    pub fn down(amount: f32) -> Self {
        PanCameraEvent { x: 0., y: -amount }
    }

    pub fn left(amount: f32) -> Self {
        PanCameraEvent { x: -amount, y: 0. }
    }
}

#[derive(Debug)]
pub struct ZoomCameraEvent {
    // (amount ∈ (1, ∞)) → zoom in
    // (amount ∈ [1] → no change
    // (amount ∈ (0, 1)) → zoom out
    pub amount: f32,
}

impl ZoomCameraEvent {
    pub fn zoom_in() -> Self {
        ZoomCameraEvent {
            amount: ZOOM_AMOUNT,
        }
    }

    pub fn zoom_out() -> Self {
        ZoomCameraEvent {
            amount: 1. / ZOOM_AMOUNT,
        }
    }
}


impl Plugin for RgisEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadGeoJsonFileEvent>()
            .add_event::<LayerLoadedEvent>()
            .add_event::<ToggleLayerVisibilityEvent>()
            .add_event::<ToggleMaterialEvent>()
            .add_event::<PanCameraEvent>()
            .add_event::<ZoomCameraEvent>()
            .add_event::<CenterCameraEvent>();
    }
}
