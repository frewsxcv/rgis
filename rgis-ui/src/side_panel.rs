use bevy_egui::egui::{self, Align, Layout, Widget};
use std::marker;

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
    render_message_event_writer:
        bevy::ecs::event::EventWriter<'w, 's, rgis_events::RenderMessageEvent>,
    open_operation_window_event_writer:
        bevy::ecs::event::EventWriter<'w, 's, crate::events::OpenOperationWindowEvent>,
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
                (AddLayerButton {
                    events: self.events,
                })
                .ui(ui);
                self.render_layers(ui);
            });
        });
    }

    fn render_layers_heading(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺 Layers");
    }

    fn render_layers(&mut self, ui: &mut egui::Ui) {
        for (i, layer) in self.layers.iter_top_to_bottom().enumerate() {
            self.render_layer(ui, layer, i > 0, i < self.layers.count() - 1);
        }
    }

    fn render_layer(
        &mut self,
        ui: &mut egui::Ui,
        layer: &rgis_layers::Layer,
        is_move_up_enabled: bool,
        is_move_down_enabled: bool,
    ) {
        egui::CollapsingHeader::new(&layer.name)
            .id_source(layer.id) // Instead of using the layer name as the ID (which is not unique), use the layer ID
            .show(ui, |ui| {
                ui.label(format!("Type: {:?}", layer.geom_type));

                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    if ui.button("✏ Manage").clicked() {
                        self.manage_layer_window_state.is_visible = true;
                        self.manage_layer_window_state.layer_id = Some(layer.id);
                    }

                    ui.horizontal(|ui| {
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
                    });

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

                    if ui.button("❌ Remove").clicked() {
                        self.delete_layer(layer);
                    }

                    egui::CollapsingHeader::new("⚙ Operations")
                        .id_source(format!("{:?}-operations", layer.id)) // Instead of using the layer name as the ID (which is not unique), use the layer ID
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                                if ui.button("Bounding rect").clicked() {
                                    if let Ok(bounding_rect) =
                                        layer.unprojected_feature_collection.bounding_rect()
                                    {
                                        if let Ok(feature_collection) = geo_projected::Unprojected::<
                                            geo_features::FeatureCollection,
                                        >::from_geometry(
                                            bounding_rect.0.into()
                                        ) {
                                            self.events.create_layer_event_writer.send(
                                                rgis_events::CreateLayerEvent {
                                                    feature_collection,           // todo
                                                    name: "Bounding rect".into(), // todo
                                                    source_crs: layer.crs.clone(),
                                                },
                                            );
                                        }
                                    }
                                }

                                OperationButton::<rgis_geo_ops::ConvexHull>::new(
                                    self.events,
                                    layer,
                                ).ui(ui);
                                OperationButton::<rgis_geo_ops::Outliers>::new(
                                    self.events,
                                    layer,
                                ).ui(ui);
                                OperationButton::<rgis_geo_ops::Simplify>::new(
                                    self.events,
                                    layer,
                                ).ui(ui);
                                OperationButton::<rgis_geo_ops::Smoothing>::new(
                                    self.events,
                                    layer,
                                ).ui(ui);
                                OperationButton::<rgis_geo_ops::Triangulate>::new(
                                    self.events,
                                    layer,
                                ).ui(ui);
                                OperationButton::<rgis_geo_ops::UnsignedArea>::new(
                                    self.events,
                                    layer,
                                ).ui(ui);
                            });
                        });
                });
            });
        ui.separator();
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

struct OperationButton<'a, 'w, 's, Op: rgis_geo_ops::OperationEntry> {
    events: &'a mut Events<'w, 's>,
    layer: &'a rgis_layers::Layer,
    operation: marker::PhantomData<Op>,
}

impl<'a, 'w, 's, Op: rgis_geo_ops::OperationEntry> OperationButton<'a, 'w, 's, Op> {
    fn new(events: &'a mut Events<'w, 's>, layer: &'a rgis_layers::Layer) -> Self {
        OperationButton {
            events,
            layer,
            operation: Default::default(),
        }
    }
}

impl<'a, 'w, 's, Op: rgis_geo_ops::OperationEntry> egui::Widget
    for OperationButton<'a, 'w, 's, Op>
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = ui.add_enabled(
            Op::ALLOWED_GEOM_TYPES.contains(self.layer.geom_type),
            egui::Button::new(Op::NAME),
        );
        if button.clicked() {
            let mut operation = Op::build();
            match operation.next_action() {
                rgis_geo_ops::Action::RenderUi => {
                    self.events.open_operation_window_event_writer.send(
                        crate::events::OpenOperationWindowEvent {
                            operation,
                            feature_collection: self.layer.unprojected_feature_collection.clone(), // TODO: clone?
                        },
                    )
                }
                rgis_geo_ops::Action::Perform => {
                    // TODO: perform in background job
                    let outcome =
                        operation.perform(self.layer.unprojected_feature_collection.clone()); // TODO: clone?

                    match outcome {
                        Ok(rgis_geo_ops::Outcome::FeatureCollection(feature_collection)) => {
                            self.events.create_layer_event_writer.send(
                                rgis_events::CreateLayerEvent {
                                    feature_collection,
                                    name: Op::NAME.into(),
                                    source_crs: self.layer.crs.clone(),
                                },
                            );
                        }
                        Ok(rgis_geo_ops::Outcome::Text(text)) => self
                            .events
                            .render_message_event_writer
                            .send(rgis_events::RenderMessageEvent(text)),
                        Err(e) => {
                            bevy::log::error!("Encountered an error during the operation: {}", e);
                        }
                    }
                }
            }
        }
        button
    }
}

struct AddLayerButton<'a, 'w, 's> {
    events: &'a mut Events<'w, 's>,
}

impl<'a, 'w, 's> egui::Widget for AddLayerButton<'a, 'w, 's> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = ui.button("➕ Add Layer");

        if button.clicked() {
            self.events
                .show_add_layer_window_event_writer
                .send_default();
        }

        button
    }
}
