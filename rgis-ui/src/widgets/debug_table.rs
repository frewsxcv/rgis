use crate::debug_window::LastDebugStats;
use bevy_egui::egui;

pub struct DebugTable<'a> {
    pub last: &'a LastDebugStats,
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
