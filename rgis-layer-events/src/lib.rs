use bevy::prelude::*;
use geo_projected::UnprojectedScalar;

#[derive(Debug, Message)]
pub struct LayerCreatedEvent(pub rgis_primitives::LayerId);

#[derive(Message, Debug)]
pub struct ToggleLayerVisibilityEvent(pub rgis_primitives::LayerId);

#[derive(Message, Debug)]
pub struct LayerBecameHiddenEvent(pub rgis_primitives::LayerId);

#[derive(Message, Debug)]
pub struct LayerBecameVisibleEvent(pub rgis_primitives::LayerId);

/// After a `Layer`'s color is changed
#[derive(Clone, Copy, Message)]
pub enum LayerColorUpdatedEvent {
    Fill(rgis_primitives::LayerId),
    Stroke(rgis_primitives::LayerId),
}

#[derive(Message, Debug)]
pub struct LayerPointSizeUpdatedEvent(pub rgis_primitives::LayerId);

#[derive(Message)]
pub struct DeleteLayerEvent(pub rgis_primitives::LayerId);

pub enum MoveDirection {
    Up,
    Down,
}

#[derive(Message)]
pub struct MoveLayerEvent(pub rgis_primitives::LayerId, pub MoveDirection);

#[derive(Message)]
pub struct LayerZIndexUpdatedEvent(pub rgis_primitives::LayerId);

#[derive(Message)]
pub struct CreateLayerEvent {
    pub feature_collection: geo_features::FeatureCollection<UnprojectedScalar>,
    pub name: String,
    pub source_crs: rgis_primitives::Crs,
}

#[derive(Message)]
pub struct LayerReprojectedEvent(pub rgis_primitives::LayerId);

#[derive(Message, Debug)]
pub struct DuplicateLayerEvent(pub rgis_primitives::LayerId);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LayerCreatedEvent>()
            .add_message::<ToggleLayerVisibilityEvent>()
            .add_message::<LayerBecameHiddenEvent>()
            .add_message::<LayerBecameVisibleEvent>()
            .add_message::<LayerColorUpdatedEvent>()
            .add_message::<LayerPointSizeUpdatedEvent>()
            .add_message::<DeleteLayerEvent>()
            .add_message::<MoveLayerEvent>()
            .add_message::<LayerZIndexUpdatedEvent>()
            .add_message::<LayerReprojectedEvent>()
            .add_message::<CreateLayerEvent>()
            .add_message::<DuplicateLayerEvent>();
    }
}
