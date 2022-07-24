use bevy_egui::egui;

// const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

#[derive(bevy::ecs::system::SystemParam)]
pub struct Events<'w, 's> {
    toggle_layer_visibility_event_writer:
        bevy::ecs::event::EventWriter<'w, 's, rgis_events::ToggleLayerVisibilityEvent>,
    center_layer_event_writer:
        bevy::ecs::event::EventWriter<'w, 's, rgis_events::CenterCameraEvent>,
    delete_layer_event_writer: bevy::ecs::event::EventWriter<'w, 's, rgis_events::DeleteLayerEvent>,
    move_layer_event_writer: bevy::ecs::event::EventWriter<'w, 's, rgis_events::MoveLayerEvent>,
}

pub(crate) struct SidePanel<'a, 'w, 's> {
    pub egui_ctx: &'a egui::Context,
    pub state: &'a mut crate::AddLayerWindowState,
    pub manage_layer_window_state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub events: &'a mut Events<'w, 's>,
}

impl<'a, 'w, 's> SidePanel<'a, 'w, 's> {
    pub(crate) fn render(&mut self) {
        let side_panel = egui::SidePanel::left("left-side-panel").resizable(true);

        let inner_response = side_panel.show(self.egui_ctx, |ui| {
            self.render_layers_window(ui);
        });

        if inner_response.response.changed() {
            bevy::log::error!("resized");
            // set height of resource?
        }
    }

    fn render_layers_window(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_layers_heading(ui);
                self.render_add_layer_button(ui);
                self.render_layers(ui);
            });
        });
    }

    fn render_layers_heading(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺 Layers");
    }

    fn render_add_layer_button(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(!self.state.is_visible, |ui| {
            if ui.button("➕ Add Layer").clicked() {
                self.state.is_visible = true;
            }
        });
    }

    fn render_layers(&mut self, ui: &mut egui::Ui) {
        for (i, layer) in self.layers.iter_top_to_bottom().enumerate() {
            self.render_layer(ui, layer, i > 0, i < self.layers.count());
        }
    }

    fn render_layer(
        &mut self,
        ui: &mut egui::Ui,
        layer: &rgis_layers::Layer,
        is_move_up_enabled: bool,
        is_move_down_enabled: bool,
    ) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            egui::CollapsingHeader::new(layer.name.to_owned())
                .id_source(layer.id) // Instead of using the layer name as the ID (which is not unique), use the layer ID
                .show(ui, |ui| {
                    if ui.button("✏ Manage").clicked() {
                        self.manage_layer_window_state.is_visible = true;
                        self.manage_layer_window_state.layer_id = Some(layer.id);
                    }

                    if ui
                        .add_enabled(is_move_up_enabled, egui::Button::new("⬆ Move up"))
                        .clicked()
                    {
                        self.events
                            .move_layer_event_writer
                            .send(rgis_events::MoveLayerEvent(
                                layer.id,
                                rgis_events::MoveDirection::Up,
                            ));
                    }

                    if ui
                        .add_enabled(is_move_down_enabled, egui::Button::new("⬇ Move down"))
                        .clicked()
                    {
                        self.events
                            .move_layer_event_writer
                            .send(rgis_events::MoveLayerEvent(
                                layer.id,
                                rgis_events::MoveDirection::Down,
                            ));
                    }

                    if layer.visible {
                        if ui.button("👁 Hide").clicked() {
                            self.events
                                .toggle_layer_visibility_event_writer
                                .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
                        }
                    } else if ui.button("👁 Show").clicked() {
                        self.events
                            .toggle_layer_visibility_event_writer
                            .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
                    }

                    if ui.button("🔎 Zoom to extent").clicked() {
                        self.events
                            .center_layer_event_writer
                            .send(rgis_events::CenterCameraEvent(layer.id))
                    }

                    if ui.button("⚙ Calculate planar area").clicked() {
                        // use geo::algorithm::area::Area;
                        // println!("{:?}", layer.projected_geometry.unsigned_area());
                    }

                    if ui.button("❌ Remove").clicked() {
                        self.events
                            .delete_layer_event_writer
                            .send(rgis_events::DeleteLayerEvent(layer.id))
                    }
                });
        });
    }
}
