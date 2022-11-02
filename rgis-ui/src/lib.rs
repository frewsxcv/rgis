#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;
use std::{collections, marker};

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

#[derive(Copy, Clone)]
pub struct SidePanelWidth(pub f32);

#[derive(Copy, Clone)]
pub struct TopPanelHeight(pub f32);

#[derive(Copy, Clone)]
pub struct BottomPanelHeight(pub f32);

#[derive(bevy::ecs::system::SystemParam)]
pub struct UiMargins<'w, 's> {
    pub left: Res<'w, SidePanelWidth>,
    pub top: Res<'w, TopPanelHeight>,
    pub bottom: Res<'w, BottomPanelHeight>,
    #[system_param(ignore)]
    marker: marker::PhantomData<&'s usize>,
}

impl<'w, 's> UiMargins<'w, 's> {
    pub fn to_ui_rect(&self) -> bevy::ui::UiRect<f32> {
        bevy::ui::UiRect {
            left: self.left.0,
            top: self.top.0,
            bottom: self.bottom.0,
            right: 0.,
        }
    }
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

pub struct DebugStatsWindowState {
    timer: Timer,
    is_visible: bool,
    history: collections::VecDeque<f64>,
}

const DEBUG_STATS_HISTORY_LEN: usize = 100;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(add_layer_window::SelectedFile(None))
            .insert_resource(ManageLayerWindowState::default())
            .insert_resource(TopPanelHeight(0.))
            .insert_resource(BottomPanelHeight(0.))
            .insert_resource(SidePanelWidth(0.))
            .insert_resource(DebugStatsWindowState {
                timer: Timer::from_seconds(0.5, true),
                is_visible: false,
                history: collections::VecDeque::with_capacity(DEBUG_STATS_HISTORY_LEN),
            });

        app.add_startup_system_set(systems::startup_system_set());

        for system_set in systems::system_sets() {
            app.add_system_set(system_set);
        }
    }
}
