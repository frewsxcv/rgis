use bevy::prelude::*;
use geo_projected::ProjectedCoord;

#[derive(Event)]
pub struct MapClickedEvent(pub ProjectedCoord);

#[derive(Clone, Copy, Event, Debug)]
pub struct FeatureSelectedEvent(pub rgis_primitives::LayerId, pub geo_features::FeatureId);

#[derive(Event)]
pub struct FeaturesDeselectedEvent;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapClickedEvent>()
            .add_event::<FeatureSelectedEvent>()
            .add_event::<FeaturesDeselectedEvent>();
    }
}
