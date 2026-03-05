use bevy::prelude::*;

#[derive(Message)]
pub struct ChangeCrsMessage {
    pub old: rgis_primitives::Crs,
    pub new: rgis_primitives::Crs,
}

#[derive(Message)]
pub struct CrsChangedMessage {
    pub old: rgis_primitives::Crs,
    pub new: rgis_primitives::Crs,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChangeCrsMessage>()
            .add_message::<CrsChangedMessage>();
    }
}
