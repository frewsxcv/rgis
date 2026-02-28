use bevy::prelude::*;
use geo_projected::ProjectedCoord;

#[derive(Message)]
pub struct MapClickedEvent(pub ProjectedCoord);

#[derive(Clone, Copy, Message, Debug)]
pub struct FeatureSelectedEvent(pub rgis_primitives::LayerId, pub geo_features::FeatureId);

#[derive(Message)]
pub struct FeaturesDeselectedEvent;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MapClickedEvent>()
            .add_message::<FeatureSelectedEvent>()
            .add_message::<FeaturesDeselectedEvent>();
    }
}
