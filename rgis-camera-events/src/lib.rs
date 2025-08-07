use bevy::prelude::*;
use geo_projected::ProjectedCoord;

// Magic number used to normalize the host's scroll value.
const ZOOM_FACTOR: f32 = 500.;

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

#[derive(Event, Debug)]
pub struct CenterCameraEvent(pub rgis_primitives::LayerId);

impl From<rgis_primitives::LayerId> for CenterCameraEvent {
    #[inline]
    fn from(layer_id: rgis_primitives::LayerId) -> Self {
        CenterCameraEvent(layer_id)
    }
}

#[derive(Default, Event, Debug)]
pub struct RecalculateMousePositionEvent;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PanCameraEvent>()
            .add_event::<ZoomCameraEvent>()
            .add_event::<CenterCameraEvent>()
            .add_event::<RecalculateMousePositionEvent>();
    }
}
