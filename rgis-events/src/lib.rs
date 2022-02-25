use bevy::prelude::*;
use std::path;

#[derive(Debug)]
pub enum LoadGeoJsonFileEvent {
    FromPath {
        path: path::PathBuf,
        source_srs: String,
        target_srs: String,
    },
    FromBytes {
        bytes: Vec<u8>,
        source_srs: String,
        target_srs: String,
    },
}

pub struct RgisEventsPlugin;

impl Plugin for RgisEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadGeoJsonFileEvent>();
    }
}
