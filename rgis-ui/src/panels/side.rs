use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui::{self, Align, Layout, Widget};
use rgis_camera_events::CenterCameraEvent;
use rgis_layer_events::{
    CreateLayerEvent, DeleteLayerEvent, MoveLayerEvent, ToggleLayerVisibilityEvent,
};
use rgis_ui_events::{ShowAddLayerWindow, ShowManageLayerWindowEvent};
use std::marker;

// const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

#[derive(SystemParam)]
pub struct Events<'w> {
    pub toggle_layer_visibility_event_writer: EventWriter<'w, ToggleLayerVisibilityEvent>,
    pub center_layer_event_writer: EventWriter<'w, CenterCameraEvent>,
    pub delete_layer_event_writer: EventWriter<'w, DeleteLayerEvent>,
    pub move_layer_event_writer: EventWriter<'w, MoveLayerEvent>,
    pub create_layer_event_writer: EventWriter<'w, CreateLayerEvent>,
    pub show_add_layer_window_event_writer: EventWriter<'w, ShowAddLayerWindow>,
    pub show_manage_layer_window_event_writer: EventWriter<'w, ShowManageLayerWindowEvent>,
    pub perform_operation_event_writer: EventWriter<'w, crate::events::PerformOperationEvent>,
}

pub struct Side<'a, 'w> {
    pub egui_ctx: &'a egui::Context,
    pub layers: &'a rgis_layers::Layers,
    pub events: &'a mut Events<'w>,
    pub side_panel_width: &'a mut rgis_units::SidePanelWidth,
}

impl Side<'_, '_> {
    pub fn render(&mut self) {
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
                ui.add(crate::widgets::add_layer::AddLayer {
                    events: self.events,
                });
                self.render_layers(ui);
            });
        });
    }

    fn render_layers_heading(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺 Layers");
    }

    fn render_layers(&mut self, ui: &mut egui::Ui) {
        for (i, layer) in self.layers.iter_top_to_bottom().enumerate() {
            ui.add(Layer {
                is_move_down_enabled: i < self.layers.count() - 1,
                is_move_up_enabled: i > 0,
                layer,
                events: self.events,
            });
            ui.separator();
        }
    }
}

pub(crate) struct OperationButton<'a, 'w, Op: rgis_geo_ops::OperationEntry> {
    events: &'a mut Events<'w>,
    layer: &'a rgis_layers::Layer,
    operation: marker::PhantomData<Op>,
}

impl<'a, 'w, Op: rgis_geo_ops::OperationEntry> OperationButton<'a, 'w, Op> {
    pub(crate) fn new(events: &'a mut Events<'w>, layer: &'a rgis_layers::Layer) -> Self {
        OperationButton {
            events,
            layer,
            operation: Default::default(),
        }
    }
}

impl<Op: rgis_geo_ops::OperationEntry> egui::Widget for OperationButton<'_, '_, Op> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = ui.add_enabled(
            Op::ALLOWED_GEOM_TYPES.contains(self.layer.geom_type),
            egui::Button::new(Op::NAME),
        );
        if button.clicked() {
            self.events.perform_operation_event_writer.write(
                crate::events::PerformOperationEvent {
                    operation: Box::new(Op::build()),
                    layer_id: self.layer.id,
                },
            );
        }
        button
    }
}

struct Layer<'a, 'w> {
    layer: &'a rgis_layers::Layer,
    is_move_up_enabled: bool,
    is_move_down_enabled: bool,
    events: &'a mut Events<'w>,
}

impl Layer<'_, '_> {
    fn delete_layer(&mut self, layer: &rgis_layers::Layer) {
        self.events
            .delete_layer_event_writer
            .write(DeleteLayerEvent(layer.id));
    }
}

impl Widget for Layer<'_, '_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let Layer {
            layer,
            is_move_up_enabled,
            is_move_down_enabled,
            events: _,
        } = self;
        egui::CollapsingHeader::new(&layer.name)
            .id_salt(layer.id) // Instead of using the layer name as the ID (which is not unique), use the layer ID
            .show(ui, |ui| {
                if !layer.is_active() {
                    ui.spinner();
                    return;
                }

                ui.label(format!("Type: {}", layer.geom_type));

                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    if ui.button("✏ Manage").clicked() {
                        self.events
                            .show_manage_layer_window_event_writer
                            .write(ShowManageLayerWindowEvent(layer.id));
                    }

                    ui.add(crate::widgets::move_up_move_down::MoveUpMoveDown {
                        layer,
                        is_move_up_enabled,
                        is_move_down_enabled,
                        events: self.events,
                    });

                    ui.add(crate::widgets::toggle_layer::ToggleLayer {
                        layer,
                        events: self.events,
                    });

                    if ui.button("🔎 Zoom to extent").clicked() {
                        self.events
                            .center_layer_event_writer
                            .write(CenterCameraEvent(layer.id));
                    }

                    if ui.button("❌ Remove").clicked() {
                        self.delete_layer(layer);
                    }

                    egui::CollapsingHeader::new("⚙ Operations")
                        .id_salt(format!("{:?}-operations", layer.id)) // Instead of using the layer name as the ID (which is not unique), use the layer ID
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                                ui.add(crate::widgets::operations::Operations {
                                    layer,
                                    events: self.events,
                                });
                            });
                        });
                });
            })
            .header_response
    }
}
