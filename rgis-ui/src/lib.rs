#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod add_layer_window;
mod bottom_panel;
mod change_crs_window;
mod feature_properties_window;
mod manage_layer_window;
mod message_window;
mod side_panel;
mod systems;
mod top_panel;

pub struct Plugin;

// TODO: most of these could be moved to individual Local or Res
#[derive(Debug, Default)]
struct UiState {
    /// Is the 'add layer' window visible?
    is_add_layer_window_visible: bool,
    is_message_window_visible: bool,
    message: Option<String>,
}

pub struct ManageLayerWindowState {
    layer_id: Option<rgis_layer_id::LayerId>,
    is_visible: bool,
}

pub struct FeaturePropertiesWindowState {
    properties: Option<geo_features::Properties>,
    is_visible: bool,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UiState {
                is_add_layer_window_visible: true,
                is_message_window_visible: false,
                message: None,
            })
            .insert_resource(ManageLayerWindowState {
                is_visible: false,
                layer_id: None,
            })
            .insert_resource(FeaturePropertiesWindowState {
                is_visible: false,
                properties: None,
            });
        for system_set in systems::system_sets() {
            app.add_system_set(system_set);
        }
    }
}
