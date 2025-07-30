use bevy::prelude::*;

mod jobs;
mod systems;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        systems::configure(app);
    }
}
