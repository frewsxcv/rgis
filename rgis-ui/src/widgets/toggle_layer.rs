use crate::panels::side::Events;
use bevy_egui::egui;
use rgis_layer_events::ToggleLayerVisibilityEvent;

pub struct ToggleLayer<'a, 'w> {
    pub layer: &'a rgis_layers::Layer,
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for ToggleLayer<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = if self.layer.visible {
            ui.button("👁 Hide")
        } else {
            ui.button("👁 Show")
        };
        crate::widget_registry::register("Toggle Visibility", button.rect);

        if button.clicked() {
            self.events
                .toggle_layer_visibility_event_writer
                .write(ToggleLayerVisibilityEvent(self.layer.id));
        }

        button
    }
}
