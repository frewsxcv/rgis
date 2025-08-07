use bevy::prelude::*;
use geo_projected::UnprojectedScalar;

#[derive(Debug, Event)]
pub struct LayerCreatedEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct ToggleLayerVisibilityEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct LayerBecameHiddenEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct LayerBecameVisibleEvent(pub rgis_primitives::LayerId);

/// After a `Layer`'s color is changed
#[derive(Clone, Copy, Event)]
pub enum LayerColorUpdatedEvent {
    Fill(rgis_primitives::LayerId),
    Stroke(rgis_primitives::LayerId),
}

#[derive(Event)]
pub struct DeleteLayerEvent(pub rgis_primitives::LayerId);

pub enum MoveDirection {
    Up,
    Down,
}

#[derive(Event)]
pub struct MoveLayerEvent(pub rgis_primitives::LayerId, pub MoveDirection);

#[derive(Event)]
pub struct LayerZIndexUpdatedEvent(pub rgis_primitives::LayerId);

#[derive(Event)]
pub struct CreateLayerEvent {
    pub feature_collection: geo_features::FeatureCollection<UnprojectedScalar>,
    pub name: String,
    pub source_crs: rgis_primitives::Crs,
}

#[derive(Event)]
pub struct LayerReprojectedEvent(pub rgis_primitives::LayerId);

#[derive(Event, Debug)]
pub struct DuplicateLayerEvent(pub rgis_primitives::LayerId);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LayerCreatedEvent>()
            .add_event::<ToggleLayerVisibilityEvent>()
            .add_event::<LayerBecameHiddenEvent>()
            .add_event::<LayerBecameVisibleEvent>()
            .add_event::<LayerColorUpdatedEvent>()
            .add_event::<DeleteLayerEvent>()
            .add_event::<MoveLayerEvent>()
            .add_event::<LayerZIndexUpdatedEvent>()
            .add_event::<LayerReprojectedEvent>()
            .add_event::<CreateLayerEvent>()
            .add_event::<DuplicateLayerEvent>();
    }
}
