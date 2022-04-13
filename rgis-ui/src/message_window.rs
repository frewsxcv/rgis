use bevy_egui::egui;

pub(crate) struct MessageWindow<'a> {
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub state: &'a mut crate::UiState,
}

impl<'a> MessageWindow<'a> {
    pub fn render(&mut self) {
        if let Some(message) = self.state.messages.pop() {
            egui::Window::new("Message Window")
                .id(egui::Id::new("Message window"))
                .open(&mut true)
                .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
                .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                    ui.label(message);
                });
                    // egui::Grid::new("manage_layer_window_grid")
                    //     .num_columns(2)
                    //     .striped(true)
                    //     .show(ui, |ui| {
                    //         ui.label("Name");
                    //         ui.label(layer.name.clone());
                    //         ui.end_row();
                    //         ui.label("CRS");
                    //         ui.label(layer.crs.clone());
                    //         ui.end_row();
                    //         ui.label("Color");
                    //         let mut old_color = layer.color.as_linear_rgba_f32();
                    //         if ui
                    //             .color_edit_button_rgba_unmultiplied(&mut old_color)
                    //             .changed()
                    //         {
                    //             self.color_events.send(rgis_events::UpdateLayerColor(
                    //                 layer.id,
                    //                 bevy::prelude::Color::rgba_linear(
                    //                     old_color[0],
                    //                     old_color[1],
                    //                     old_color[2],
                    //                     old_color[3],
                    //                 ),
                    //             ));
                    //         }
                    //         ui.end_row();
                    //     });
                // });
        }
    }
}
