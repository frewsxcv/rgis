use bevy::prelude::*;

mod events;
mod panels;
mod systems;
mod widgets;
mod windows;

pub struct Plugin;

#[derive(Default, Resource)]
pub struct MessageWindowState {
    is_visible: bool,
    message: Option<String>,
}

#[derive(Default, Resource)]
pub struct ManageLayerWindowState {
    layer_id: Option<rgis_primitives::LayerId>,
    is_visible: bool,
}

#[derive(Default)]
pub struct FeaturePropertiesWindowState {
    layer_id: Option<rgis_primitives::LayerId>,
    properties: Option<geo_features::Properties>,
    is_visible: bool,
}

#[derive(Resource, Default)]
pub struct ChangeCrsWindowState {
    pub is_visible: bool,
}

#[derive(Default)]
struct OperationWindowState {
    is_visible: bool,
    operation: Option<Box<dyn Send + Sync + rgis_geo_ops::Operation>>,
    feature_collection: Option<geo_features::FeatureCollection<geo_projected::UnprojectedScalar>>,
    source_crs: Option<rgis_primitives::Crs>,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin::default())
            .insert_resource(windows::add_layer::SelectedFile(None))
            .insert_resource(rgis_units::TopPanelHeight(0.))
            .insert_resource(rgis_units::BottomPanelHeight(0.))
            .insert_resource(rgis_units::SidePanelWidth(0.))
            .insert_resource(ChangeCrsWindowState::default())
            .add_event::<events::OpenOperationWindowEvent>();

        systems::configure(app);
    }
}
