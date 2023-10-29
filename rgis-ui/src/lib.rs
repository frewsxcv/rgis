#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;
use bevy_egui::egui;
use std::{collections, marker};

mod add_layer_window;
mod bottom_panel;
mod change_crs_window;
mod events;
mod feature_properties_window;
mod manage_layer_window;
mod message_window;
mod operation_window;
mod side_panel;
mod systems;
mod top_panel;
mod widgets;

trait Window: egui::Widget {
    // type State: Resource;

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    // fn state(&self) -> &Self::State;
    // fn state_mut(&mut self) -> &mut Self::State;
}

struct DebugWindow {
    state: DebugStatsWindowState,
}

const FPS_MAX: f64 = 100.;

impl egui::Widget for DebugWindow {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        if self.state.history.is_empty() || self.state.timer.tick(time.delta()).just_finished() {
            let fps = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|d| d.measurement())
                .map(|m| m.value);
            let frame_time = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                .and_then(|d| d.measurement())
                .map(|m| m.value);
            let frame_count = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .and_then(|d| d.measurement())
                .map(|m| m.value);

            if let Some(fps) = fps {
                self.state.history.push_back(fps);
                last.fps = fps;
            }
            if let Some(frame_time) = frame_time {
                last.frame_time = frame_time;
            }
            if let Some(frame_count) = frame_count {
                last.frame_count = frame_count;
            }

            if self.state.history.len() >= crate::DEBUG_STATS_HISTORY_LEN {
                let _ = self.state.history.pop_front();
            }
        }

        let sin = if self.state.is_visible {
            self.state
                .history
                .iter()
                .enumerate()
                .map(|(x, y)| egui_plot::PlotPoint::new(x as f64, y.min(FPS_MAX)))
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        egui::Window::new("Debug")
            .default_width(200.)
            .open(&mut self.state.is_visible)
            .show(ui.ctx(), move |ui| {
                DebugTable { last: &last }.ui(ui);

                use egui_plot::{Line, Plot, PlotPoints};
                let line = Line::new(PlotPoints::Owned(sin));
                Plot::new("fps_plot")
                    .allow_drag(false)
                    .allow_boxed_zoom(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .set_margin_fraction((0., 0.).into())
                    .show_x(false)
                    .x_axis_formatter(|_, _, _| "".into())
                    .y_axis_formatter(|n, _, _| format!("{n:?}"))
                    .include_x(0.)
                    .include_x(crate::DEBUG_STATS_HISTORY_LEN as f64)
                    .include_y(0.)
                    .include_y(FPS_MAX)
                    .view_aspect(2.) // Width is twice as big as height
                    .show(ui, |plot_ui| plot_ui.line(line));
            })
    }
}

impl Window for DebugWindow {
    // type State = DebugStatsWindowState;

    fn is_visible(&self) -> bool {
        self.state.is_visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.state.is_visible = visible;
    }
}

pub struct Plugin;

#[derive(Copy, Clone, Resource)]
pub struct SidePanelWidth(pub f32);

#[derive(Copy, Clone, Resource)]
pub struct TopPanelHeight(pub f32);

#[derive(Copy, Clone, Resource)]
pub struct BottomPanelHeight(pub f32);

#[derive(bevy::ecs::system::SystemParam, Resource)]
pub struct UiMargins<'w, 's> {
    pub left: Res<'w, SidePanelWidth>,
    pub top: Res<'w, TopPanelHeight>,
    pub bottom: Res<'w, BottomPanelHeight>,
    #[system_param(ignore)]
    marker: marker::PhantomData<&'s usize>,
}

impl<'w, 's> UiMargins<'w, 's> {
    // pub fn to_ui_rect(&self) -> bevy::ui::UiRect {
    //     bevy::ui::UiRect {
    //         left: Val::Px(self.left.0),
    //         top: Val::Px(self.top.0),
    //         bottom: Val::Px(self.bottom.0),
    //         right: Val::Px(0.),
    //     }
    // }
}

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
    properties: Option<geo_features::Properties>,
    is_visible: bool,
}

#[derive(Resource)]
pub struct DebugStatsWindowState {
    timer: Timer,
    is_visible: bool,
    history: collections::VecDeque<f64>,
}

#[derive(Default)]
struct OperationWindowState {
    is_visible: bool,
    operation: Option<Box<dyn Send + Sync + rgis_geo_ops::Operation>>,
    feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
}

const DEBUG_STATS_HISTORY_LEN: usize = 100;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin)
            .insert_resource(add_layer_window::SelectedFile(None))
            .insert_resource(TopPanelHeight(0.))
            .insert_resource(BottomPanelHeight(0.))
            .insert_resource(SidePanelWidth(0.))
            .insert_resource(DebugStatsWindowState {
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                is_visible: false,
                history: collections::VecDeque::with_capacity(DEBUG_STATS_HISTORY_LEN),
            })
            .add_event::<events::OpenOperationWindowEvent>();

        systems::configure(app);
    }
}
