use crate::panels::side_panel::Events;
use bevy_egui::egui;

pub struct ToggleLayerWidget<'a, 'w> {
    pub layer: &'a rgis_layers::Layer,
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for ToggleLayerWidget<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = if self.layer.visible {
            ui.button("👁 Hide")
        } else {
            ui.button("👁 Show")
        };

        if button.clicked() {
            self.events
                .toggle_layer_visibility_event_writer
                .write(rgis_events::ToggleLayerVisibilityEvent(self.layer.id));
        }

        button
    }
}
