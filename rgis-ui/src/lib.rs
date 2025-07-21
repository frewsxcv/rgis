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
mod debug_window;
mod events;
mod feature_properties_window;
mod manage_layer_window;
mod message_window;
mod operation_window;
mod side_panel;
mod systems;
mod top_panel;
mod welcome_window;
mod widgets;

pub(crate) mod window;

pub struct Plugin;

#[derive(Default, Resource)]
pub struct MessageWindowState {
    is_visible: bool,
    message: Option<String>,
}

#[derive(Default, Resource)]
pub struct ManageLayerWindowState {
    layer_id: Option<rgis_layer_id::LayerId>,
    is_visible: bool,
}

#[derive(Default)]
pub struct FeaturePropertiesWindowState {
    layer_id: Option<rgis_layer_id::LayerId>,
    properties: Option<geo_features::Properties>,
    is_visible: bool,
}

#[derive(Default)]
struct OperationWindowState {
    is_visible: bool,
    operation: Option<Box<dyn Send + Sync + rgis_geo_ops::Operation>>,
    feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin::default())
            .insert_resource(add_layer_window::SelectedFile(None))
            .insert_resource(rgis_units::TopPanelHeight(0.))
            .insert_resource(rgis_units::BottomPanelHeight(0.))
            .insert_resource(rgis_units::SidePanelWidth(0.))
            .add_event::<events::OpenOperationWindowEvent>();

        systems::configure(app);
    }
}
