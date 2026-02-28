use bevy::prelude::*;

#[derive(Message, Debug)]
pub enum LoadFileEvent {
    FromNetwork {
        name: String,
        url: String,
        source_crs: rgis_primitives::Crs,
    },
    FromBytes {
        file_name: String,
        file_format: geo_file_loader::FileFormat,
        bytes: bytes::Bytes,
        source_crs: rgis_primitives::Crs,
    },
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadFileEvent>();
    }
}
