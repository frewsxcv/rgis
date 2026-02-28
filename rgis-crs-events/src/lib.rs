use bevy::prelude::*;

#[derive(Message)]
pub struct ChangeCrsEvent {
    pub old: rgis_primitives::Crs,
    pub new: rgis_primitives::Crs,
}

#[derive(Message)]
pub struct CrsChangedEvent {
    pub old: rgis_primitives::Crs,
    pub new: rgis_primitives::Crs,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChangeCrsEvent>()
            .add_message::<CrsChangedEvent>();
    }
}
