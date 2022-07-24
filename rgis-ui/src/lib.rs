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

#[derive(Default)]
pub struct AddLayerWindowState {
    is_visible: bool,
}

#[derive(Default)]
pub struct MessageWindowState {
    is_visible: bool,
    message: Option<String>,
}

#[derive(Default)]
pub struct ManageLayerWindowState {
    layer_id: Option<rgis_layer_id::LayerId>,
    is_visible: bool,
}

#[derive(Default)]
pub struct FeaturePropertiesWindowState {
    properties: Option<geo_features::Properties>,
    is_visible: bool,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(AddLayerWindowState { is_visible: true })
            .insert_resource(MessageWindowState::default())
            .insert_resource(ManageLayerWindowState::default())
            .insert_resource(FeaturePropertiesWindowState::default());

        for system_set in systems::system_sets() {
            app.add_system_set(system_set);
        }
    }
}
