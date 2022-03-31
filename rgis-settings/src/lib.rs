#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

static DEFAULT_TARGET_CRS: &str = "EPSG:3857";

pub struct RgisSettings {
    pub target_crs: String,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RgisSettings {
            target_crs: DEFAULT_TARGET_CRS.into(),
        });
    }
}
