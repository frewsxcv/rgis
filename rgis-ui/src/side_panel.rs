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
    create_layer_event_writer: bevy::ecs::event::EventWriter<'w, 's, rgis_events::CreateLayerEvent>,
    show_add_layer_window_event_writer:
        bevy::ecs::event::EventWriter<'w, 's, rgis_events::ShowAddLayerWindow>,
}

pub(crate) struct SidePanel<'a, 'w, 's> {
    pub egui_ctx: &'a egui::Context,
    pub manage_layer_window_state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub events: &'a mut Events<'w, 's>,
    pub side_panel_width: &'a mut crate::SidePanelWidth,
}

impl<'a, 'w, 's> SidePanel<'a, 'w, 's> {
    pub(crate) fn render(&mut self) {
        let side_panel = egui::SidePanel::left("left-side-panel").resizable(true);

        let inner_response = side_panel.show(self.egui_ctx, |ui| {
            self.render_layers_window(ui);
        });

        self.side_panel_width.0 = inner_response.response.rect.width();
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
        if ui.button("➕ Add Layer").clicked() {
            self.events
                .show_add_layer_window_event_writer
                .send_default();
        }
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
                    // TODO: `geom_type` shouldn't be recalculatd every frame
                    ui.label(format!("Type: {:?}", layer.geom_type()));

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
                            self.toggle_layer_visibility(layer);
                        }
                    } else if ui.button("👁 Show").clicked() {
                        self.toggle_layer_visibility(layer);
                    }

                    if ui.button("🔎 Zoom to extent").clicked() {
                        self.events
                            .center_layer_event_writer
                            .send(rgis_events::CenterCameraEvent(layer.id))
                    }

                    if ui.button("⚙ Generate bounding rect").clicked() {
                        if let Ok(bounding_rect) =
                            layer.unprojected_feature_collection.bounding_rect()
                        {
                            if let Ok(feature_collection) =
                                geo_features::FeatureCollection::from_geometry(bounding_rect.into())
                            {
                                self.events.create_layer_event_writer.send(
                                    rgis_events::CreateLayerEvent {
                                        unprojected_geometry: feature_collection, // todo
                                        name: "Bounding rect".into(),             // todo
                                        source_crs: layer.crs.clone(),
                                    },
                                );
                            }
                        }
                    }

                    fn display_operation(
                        events: &mut Events,
                        layer: &rgis_layers::Layer,
                        operation: impl rgis_geo_ops::Operation,
                        ui: &mut egui::Ui,
                    ) {
                        // TODO: disable button if geometry isn't allowed
                        let operation_name = operation.name().to_owned();
                        if ui.button(format!("⚙ {}", operation.name())).clicked() {
                            let outcome =
                                operation.perform(layer.unprojected_feature_collection.clone()); // TODO: clone?

                            if let Some(feature_collection) = outcome {
                                events.create_layer_event_writer.send(
                                    rgis_events::CreateLayerEvent {
                                        unprojected_geometry: feature_collection,
                                        name: operation_name,
                                        source_crs: layer.crs.clone(),
                                    },
                                );
                            }
                        }
                    }

                    display_operation(self.events, layer, rgis_geo_ops::ConvexHull::default(), ui);
                    display_operation(self.events, layer, rgis_geo_ops::Outliers::default(), ui);


                    /*
                    // TODO: only enable this button for multipoint
                    if ui.button("⚙ Remove outliers").clicked() {
                        if let Ok(new_multi_point) =
                            layer.unprojected_feature_collection.remove_outliers()
                        {
                            self.events.create_layer_event_writer.send(
                                rgis_events::CreateLayerEvent {
                                    unprojected_geometry:
                                        geo_features::FeatureCollection::from_geometry(
                                            new_multi_point.into(),
                                        )
                                        .unwrap(),
                                    name: "No outliers".into(), // todo
                                    source_crs: layer.crs.clone(),
                                },
                                // TODO: Need to repaint after this is done
                            );
                        }
                    }

                    if ui.button("⚙ Generate convex hull").clicked() {
                        let hull = layer.unprojected_feature_collection.convex_hull();
                        if let Ok(feature_collection) =
                            geo_features::FeatureCollection::from_geometry(hull.into())
                        {
                            self.events.create_layer_event_writer.send(
                                rgis_events::CreateLayerEvent {
                                    unprojected_geometry: feature_collection, // todo
                                    name: "Convex hull".into(),               // todo
                                    source_crs: layer.crs.clone(),
                                },
                            );
                        }
                    }
                    */

                    if ui.button("❌ Remove").clicked() {
                        self.delete_layer(layer);
                    }
                });
        });
    }

    fn toggle_layer_visibility(&mut self, layer: &rgis_layers::Layer) {
        self.events
            .toggle_layer_visibility_event_writer
            .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
    }

    fn delete_layer(&mut self, layer: &rgis_layers::Layer) {
        self.events
            .delete_layer_event_writer
            .send(rgis_events::DeleteLayerEvent(layer.id));
    }
}
