use bevy::prelude::*;

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

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeCrsEvent>()
            .add_event::<CrsChangedEvent>();
    }
}
