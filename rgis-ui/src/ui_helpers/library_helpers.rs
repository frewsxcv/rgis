use rgis_geodesy::GeodesyContext;
use crate::windows::add_layer::AddLayerOutput;
use bevy::prelude::error;

pub fn add_from_library(
    geodesy_ctx: &GeodesyContext,
    entry: &rgis_library::Entry,
    folder: &rgis_library::Folder,
) -> Option<AddLayerOutput> {
    match geodesy_ctx.0.write() {
        Ok(mut geodesy_ctx) => {
            match rgis_geodesy::epsg_code_to_geodesy_op_handle(
                &mut *geodesy_ctx,
                entry.crs,
            ) {
                Ok(op_handle) => {
                    Some(AddLayerOutput::LoadFromLibrary {
                        name: format!("{}: {}", folder.name, entry.name),
                        url: entry.url.into(),
                        source_crs: rgis_primitives::Crs {
                            epsg_code: entry.crs,
                            op_handle,
                        },
                    })
                }
                Err(e) => {
                    error!(
                        "Failed to get geodesy op handle for EPSG:{}: {:?}",
                        entry.crs, e
                    );
                    None
                }
            }
        }
        Err(e) => {
            error!("Failed to acquire write lock on geodesy context: {}", e);
            None
        }
    }
}
