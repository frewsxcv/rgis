#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use std::marker;

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

trait Window: egui::Widget + SystemParam + Send + Sync {
    type Item<'world, 'state>: Window<State = Self::State>;

    fn title(&self) -> &str;
    fn default_width(&self) -> f32;
    fn default_anchor(&self) -> (egui::Align2, [f32; 2]) {
        (egui::Align2::LEFT_TOP, [0., 0.])
    }
}

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
        app.add_plugins(bevy_egui::EguiPlugin)
            .insert_resource(add_layer_window::SelectedFile(None))
            .insert_resource(rgis_units::TopPanelHeight(0.))
            .insert_resource(rgis_units::BottomPanelHeight(0.))
            .insert_resource(rgis_units::SidePanelWidth(0.))
            .add_event::<events::OpenOperationWindowEvent>();

        systems::configure(app);
    }
}

#[derive(Resource)]
pub(crate) struct IsWindowOpen<W: Window + Send + Sync>(pub bool, marker::PhantomData<W>);

impl<W: Window + Send + Sync> IsWindowOpen<W> {
    fn closed() -> Self {
        Self(false, marker::PhantomData)
    }

    fn open() -> Self {
        Self(true, marker::PhantomData)
    }
}
