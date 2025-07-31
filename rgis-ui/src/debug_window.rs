use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections;

#[derive(Default)]
struct LastDebugStats {
    fps: f64,
    frame_time: f64,
    frame_count: f64,
}

pub struct DebugStatsWindowState {
    pub timer: Timer,
    pub history: collections::VecDeque<f64>,
}

const DEBUG_STATS_HISTORY_LEN: usize = 100;

impl Default for DebugStatsWindowState {
    fn default() -> Self {
        DebugStatsWindowState {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            history: collections::VecDeque::with_capacity(DEBUG_STATS_HISTORY_LEN),
        }
    }
}

#[derive(SystemParam)]
pub struct DebugWindow<'w, 's> {
    diagnostics: Res<'w, bevy::diagnostic::DiagnosticsStore>,
    state: Local<'s, DebugStatsWindowState>,
    time: Res<'w, Time>,
    last: Local<'s, LastDebugStats>,
}

const FPS_MAX: f64 = 100.;

impl egui::Widget for DebugWindow<'_, '_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        if self.state.history.is_empty() || self.state.timer.tick(self.time.delta()).just_finished()
        {
            self.update_stats();
        }

        self.render_debug_table(ui);
        self.render_fps_plot(ui)
    }
}

impl DebugWindow<'_, '_> {
    fn update_stats(&mut self) {
        let fps = self
            .diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.measurement())
            .map(|m| m.value);
        let frame_time = self
            .diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(|d| d.measurement())
            .map(|m| m.value);
        let frame_count = self
            .diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            .and_then(|d| d.measurement())
            .map(|m| m.value);

        if let Some(fps) = fps {
            self.state.history.push_back(fps);
            self.last.fps = fps;
        }
        if let Some(frame_time) = frame_time {
            self.last.frame_time = frame_time;
        }
        if let Some(frame_count) = frame_count {
            self.last.frame_count = frame_count;
        }

        if self.state.history.len() >= DEBUG_STATS_HISTORY_LEN {
            let _ = self.state.history.pop_front();
        }
    }

    fn render_debug_table(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add(DebugTable { last: &self.last });
        });
    }

    fn render_fps_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        let sin = self
            .state
            .history
            .iter()
            .enumerate()
            .map(|(x, y)| egui_plot::PlotPoint::new(x as f64, y.min(FPS_MAX)))
            .collect::<Vec<_>>();

        ui.vertical(|ui| {
            let line = Line::new("fps", PlotPoints::Owned(sin));
            Plot::new("fps_plot")
                .allow_drag(false)
                .allow_boxed_zoom(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .set_margin_fraction((0., 0.).into())
                .show_x(false)
                .x_axis_formatter(|_, _| "".into())
                .y_axis_formatter(|n, _| format!("{n:?}"))
                .include_x(0.)
                .include_x(DEBUG_STATS_HISTORY_LEN as f64)
                .include_y(0.)
                .include_y(FPS_MAX)
                .view_aspect(2.) // Width is twice as big as height
                .show(ui, |plot_ui| plot_ui.line(line));
        })
        .response
    }
}

impl bevy_egui_window::Window for DebugWindow<'_, '_> {
    type Item<'w, 's> = DebugWindow<'w, 's>;

    fn title(&self) -> &str {
        "Debug"
    }

    fn default_width(&self) -> f32 {
        200.
    }
}

struct DebugTable<'a> {
    last: &'a LastDebugStats,
}

impl egui::Widget for DebugTable<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("some_unique_id")
            .striped(true)
            .show(ui, |ui| {
                ui.label("Metric");
                ui.label("Value");
                ui.end_row();

                ui.label("FPS");
                ui.label(format!("{:.2} frames/sec.", self.last.fps));
                ui.end_row();

                ui.label("Frame time");
                ui.label(format!("{:.3} sec.", self.last.frame_time));
                ui.end_row();

                ui.label("Frame count");
                ui.label(format!("{} frames", self.last.frame_count));
                ui.end_row();
            })
            .response
    }
}
