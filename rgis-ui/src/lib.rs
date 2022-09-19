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

pub struct SidePanelWidth(pub f32);

pub struct TopPanelHeight(pub f32);

pub struct BottomPanelHeight(pub f32);

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
            .insert_resource(add_layer_window::SelectedFile(None))
            .insert_resource(ManageLayerWindowState::default())
            .insert_resource(FeaturePropertiesWindowState::default())
            .insert_resource(TopPanelHeight(0.))
            .insert_resource(BottomPanelHeight(0.))
            .insert_resource(SidePanelWidth(0.));

        app.add_startup_system_set(systems::startup_system_set());

        for system_set in systems::system_sets() {
            app.add_system_set(system_set);
        }
    }
}
