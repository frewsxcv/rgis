use crate::panels::side::Events;
use bevy_egui::egui;

pub struct AddLayer<'a, 'w> {
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for AddLayer<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = ui.button("➕ Add Layer");

        if button.clicked() {
            self.events
                .show_add_layer_window_event_writer
                .write_default();
        }

        button
    }
}
