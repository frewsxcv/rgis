use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::egui;

#[derive(SystemParam)]
pub struct Welcome<'w> {
    show_add_layer_window_event_writer: MessageWriter<'w, rgis_ui_events::ShowAddLayerWindow>,
}

impl egui::Widget for Welcome<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Welcome to rgis");
            ui.label("A geospatial data viewer written in Rust.");
            ui.add_space(8.0);
            if ui.button("Add Layer...").clicked() {
                self.show_add_layer_window_event_writer.write_default();
            }
        })
        .response
    }
}

impl bevy_egui_window::Window for Welcome<'_> {
    type Item<'w, 's> = Welcome<'w>;
    const INITIALLY_OPEN: bool = true;

    fn title(&self) -> &str {
        "Welcome"
    }

    fn default_width(&self) -> f32 {
        350.0
    }

    fn default_anchor(&self) -> (egui::Align2, [f32; 2]) {
        (egui::Align2::CENTER_CENTER, [0., 0.])
    }
}
