use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui::{self, Align, Layout, Widget};
use rgis_camera_events::CenterCameraEvent;
use rgis_layer_events::{
    CreateLayerEvent, DeleteLayerEvent, MoveDirection, MoveLayerEvent,
    ToggleLayerVisibilityEvent,
};
use rgis_ui_events::{ShowAddLayerWindow, ShowManageLayerWindowEvent};
use std::marker;

// const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

#[derive(SystemParam)]
pub struct Events<'w> {
    pub toggle_layer_visibility_event_writer: MessageWriter<'w, ToggleLayerVisibilityEvent>,
    pub center_layer_event_writer: MessageWriter<'w, CenterCameraEvent>,
    pub delete_layer_event_writer: MessageWriter<'w, DeleteLayerEvent>,
    pub move_layer_event_writer: MessageWriter<'w, MoveLayerEvent>,
    pub create_layer_event_writer: MessageWriter<'w, CreateLayerEvent>,
    pub show_add_layer_window_event_writer: MessageWriter<'w, ShowAddLayerWindow>,
    pub show_manage_layer_window_event_writer: MessageWriter<'w, ShowManageLayerWindowEvent>,
    pub perform_operation_event_writer: MessageWriter<'w, rgis_ui_events::PerformOperationEvent>,
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
        ui.heading("Layers");
    }

    fn render_layers(&mut self, ui: &mut egui::Ui) {
        for (i, layer) in self.layers.iter_top_to_bottom().enumerate() {
            ui.add(Layer {
                is_move_down_enabled: i < self.layers.count() - 1,
                is_move_up_enabled: i > 0,
                layer,
                events: self.events,
            });
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
                rgis_ui_events::PerformOperationEvent {
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

fn bevy_color_to_egui_color(color: Color) -> egui::Color32 {
    let srgba: bevy::color::Srgba = color.into();
    egui::Color32::from_rgb(
        (srgba.red * 255.0) as u8,
        (srgba.green * 255.0) as u8,
        (srgba.blue * 255.0) as u8,
    )
}

impl Widget for Layer<'_, '_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let Layer {
            layer,
            is_move_up_enabled,
            is_move_down_enabled,
            events: _,
        } = self;

        let header_text = if layer.is_active() {
            layer.name.clone()
        } else {
            format!("{} (loading...)", layer.name)
        };

        let response = egui::CollapsingHeader::new(&header_text)
            .id_salt(format!("{:?}", layer.id))
            .show(ui, |ui| {
                // Visibility toggle
                let mut visible = layer.visible;
                let visibility_checkbox = ui.checkbox(&mut visible, "Visible");
                crate::widget_registry::register("Toggle Visibility", visibility_checkbox.rect);
                if visibility_checkbox.changed() {
                    self.events
                        .toggle_layer_visibility_event_writer
                        .write(ToggleLayerVisibilityEvent(layer.id));
                }

                // Color swatch with label
                let swatch_color = layer.color.fill.unwrap_or(layer.color.stroke);
                let egui_color = bevy_color_to_egui_color(swatch_color);
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::Vec2::splat(12.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, egui_color);
                    ui.label(format!("Type: {}", layer.geom_type));
                });

                ui.separator();

                let manage_btn = ui.button("Manage...");
                crate::widget_registry::register("Manage", manage_btn.rect);
                if manage_btn.clicked() {
                    self.events
                        .show_manage_layer_window_event_writer
                        .write(ShowManageLayerWindowEvent(layer.id));
                }

                let zoom_btn = ui.button("Zoom to Extent");
                crate::widget_registry::register("Zoom to extent", zoom_btn.rect);
                if zoom_btn.clicked() {
                    self.events
                        .center_layer_event_writer
                        .write(CenterCameraEvent(layer.id));
                }

                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(is_move_up_enabled, egui::Button::new("Move Up"))
                        .clicked()
                    {
                        self.events
                            .move_layer_event_writer
                            .write(MoveLayerEvent(layer.id, MoveDirection::Up));
                    }

                    if ui
                        .add_enabled(is_move_down_enabled, egui::Button::new("Move Down"))
                        .clicked()
                    {
                        self.events
                            .move_layer_event_writer
                            .write(MoveLayerEvent(layer.id, MoveDirection::Down));
                    }
                });

                ui.separator();

                let ops_header = egui::CollapsingHeader::new("Operations")
                    .id_salt(format!("{:?}-operations", layer.id))
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                            ui.add(crate::widgets::operations::Operations {
                                layer,
                                events: self.events,
                            });
                        });
                    });
                crate::widget_registry::register("Operations", ops_header.header_response.rect);

                ui.separator();

                let remove_btn = ui.button("Remove");
                crate::widget_registry::register("Remove", remove_btn.rect);
                if remove_btn.clicked() {
                    self.delete_layer(layer);
                }
            });

        crate::widget_registry::register(&layer.name, response.header_response.rect);
        response.header_response
    }
}
