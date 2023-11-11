#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod systems;

static DEFAULT_TARGET_CRS: u16 = 3857;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tool {
    Pan,
    Query,
}

#[derive(Resource)]
pub struct RgisSettings {
    pub target_crs_epsg_code: u16,
    pub current_tool: Tool,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RgisSettings {
            target_crs_epsg_code: DEFAULT_TARGET_CRS,
            current_tool: Tool::Pan,
        })
        .add_systems(Update, systems::handle_crs_changed_events);
    }
}
