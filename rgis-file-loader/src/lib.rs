#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;

mod geojson;
mod systems;
mod tasks;

trait FileLoader {
    type Error;

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error>;
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(systems::system_set());
        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(systems::load_layers_from_cli);
    }
}
