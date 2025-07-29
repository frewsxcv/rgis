#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod systems;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tool {
    Pan,
    Query,
}

#[derive(Resource)]
pub struct RgisSettings {
    pub current_tool: Tool,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RgisSettings {
            current_tool: Tool::Pan,
        });
    }
}
