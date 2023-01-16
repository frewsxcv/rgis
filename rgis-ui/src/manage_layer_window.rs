use bevy_egui::egui;

pub(crate) struct ManageLayerWindow<'a> {
    pub state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub color_events: &'a mut bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>,
}

impl<'a> ManageLayerWindow<'a> {
    pub(crate) fn render(&mut self) {
        let (true, Some(layer_id)) = (self.state.is_visible, self.state.layer_id) else {
            return;
        };
        let Some(layer) = self.layers.get(layer_id) else {
            bevy::log::warn!(
                "Could not find layer with ID {:?}, closing manage layer window",
                layer_id
            );
            self.state.is_visible = false;
            return;
        };
        egui::Window::new("Manage Layer")
            .open(&mut self.state.is_visible)
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                egui::Grid::new("manage_layer_window_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.label(&layer.name);
                        ui.end_row();
                        ui.label("CRS");
                        ui.label(&layer.crs);
                        ui.end_row();
                        ui.label("Color");
                        let mut old_color = layer.color.as_linear_rgba_f32();
                        if ui
                            .color_edit_button_rgba_unmultiplied(&mut old_color)
                            .changed()
                        {
                            self.color_events
                                .send(rgis_events::UpdateLayerColorEvent::Fill(
                                    layer.id,
                                    bevy::prelude::Color::rgba_linear(
                                        old_color[0],
                                        old_color[1],
                                        old_color[2],
                                        old_color[3],
                                    ),
                                ));
                        }
                        ui.end_row();
                    });
            });
    }
}
