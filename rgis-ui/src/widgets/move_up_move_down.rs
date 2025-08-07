use crate::panels::side::Events;
use bevy_egui::egui;
use rgis_layer_events::MoveDirection;

pub struct MoveUpMoveDown<'a, 'w> {
    pub layer: &'a rgis_layers::Layer,
    pub is_move_up_enabled: bool,
    pub is_move_down_enabled: bool,
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for MoveUpMoveDown<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.is_move_up_enabled, egui::Button::new("⬆ Move up"))
                .clicked()
            {
                self.events
                    .move_layer_event_writer
                    .write(rgis_layer_events::MoveLayerEvent(
                        self.layer.id,
                        MoveDirection::Up,
                    ));
            }

            if ui
                .add_enabled(self.is_move_down_enabled, egui::Button::new("⬇ Move down"))
                .clicked()
            {
                self.events
                    .move_layer_event_writer
                    .write(rgis_layer_events::MoveLayerEvent(
                        self.layer.id,
                        MoveDirection::Down,
                    ));
            }
        })
        .response
    }
}
