use bevy::prelude::*;

#[derive(Clone, Copy, Message, Debug)]
pub struct FeatureSelectedMessage(pub rgis_primitives::LayerId, pub geo_features::FeatureId);

#[derive(Message)]
pub struct FeaturesDeselectedMessage;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FeatureSelectedMessage>()
            .add_message::<FeaturesDeselectedMessage>();
    }
}
