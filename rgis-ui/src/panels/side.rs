use bevy_egui::egui::{self, Align, Layout};
use rgis_layer_messages::MoveDirection;
use std::marker;

pub enum SidePanelAction {
    ToggleLayerVisibility(rgis_primitives::LayerId),
    DeleteLayer(rgis_primitives::LayerId),
    MoveLayer(rgis_primitives::LayerId, MoveDirection),
    CenterCamera(rgis_primitives::LayerId),
    ShowAddLayerWindow,
    ShowManageLayerWindow(rgis_primitives::LayerId),
    PerformOperation {
        operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
        layer_id: rgis_primitives::LayerId,
    },
    CreateLayer {
        feature_collection:
            geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
        name: String,
        source_crs: rgis_primitives::Crs,
    },
}

pub struct Side<'a> {
    pub egui_ctx: &'a egui::Context,
    pub layers: &'a rgis_layers::Layers,
    pub side_panel_width: &'a mut rgis_units::SidePanelWidth,
}

impl Side<'_> {
    pub fn render(&mut self) -> Vec<SidePanelAction> {
        let mut actions = vec![];
        let side_panel = egui::SidePanel::left("left-side-panel").resizable(true);

        let inner_response = side_panel.show(self.egui_ctx, |ui| {
            self.render_layers_window(ui, &mut actions);
        });

        self.side_panel_width.0 = inner_response.response.rect.width();
        actions
    }

    fn render_layers_window(&mut self, ui: &mut egui::Ui, actions: &mut Vec<SidePanelAction>) {
        ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Layers");
                let add_layer_btn = ui.add(crate::widgets::add_layer::AddLayer);
                if add_layer_btn.clicked() {
                    actions.push(SidePanelAction::ShowAddLayerWindow);
                }
                self.render_layers(ui, actions);
            });
        });
    }

    fn render_layers(&mut self, ui: &mut egui::Ui, actions: &mut Vec<SidePanelAction>) {
        for (i, layer) in self.layers.iter_top_to_bottom().enumerate() {
            Layer {
                is_move_down_enabled: i < self.layers.count() - 1,
                is_move_up_enabled: i > 0,
                layer,
            }
            .show(ui, actions);
        }
    }
}

pub(crate) struct OperationButton<'a, Op: rgis_geo_ops::OperationEntry> {
    layer: &'a rgis_layers::Layer,
    operation: marker::PhantomData<Op>,
}

impl<'a, Op: rgis_geo_ops::OperationEntry> OperationButton<'a, Op> {
    pub(crate) fn new(layer: &'a rgis_layers::Layer) -> Self {
        OperationButton {
            layer,
            operation: Default::default(),
        }
    }
}

impl<Op: rgis_geo_ops::OperationEntry> egui::Widget for OperationButton<'_, Op> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let enabled = self
            .layer
            .geom_type()
            .map(|gt| Op::ALLOWED_GEOM_TYPES.contains(gt))
            .unwrap_or(false);
        ui.add_enabled(enabled, egui::Button::new(Op::NAME))
    }
}

struct Layer<'a> {
    layer: &'a rgis_layers::Layer,
    is_move_up_enabled: bool,
    is_move_down_enabled: bool,
}

fn bevy_color_to_egui_color(color: bevy::prelude::Color) -> egui::Color32 {
    let srgba: bevy::color::Srgba = color.into();
    egui::Color32::from_rgb(
        (srgba.red * 255.0) as u8,
        (srgba.green * 255.0) as u8,
        (srgba.blue * 255.0) as u8,
    )
}

impl Layer<'_> {
    fn show(self, ui: &mut egui::Ui, actions: &mut Vec<SidePanelAction>) -> egui::Response {
        let Layer {
            layer,
            is_move_up_enabled,
            is_move_down_enabled,
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
                    actions.push(SidePanelAction::ToggleLayerVisibility(layer.id));
                }

                // Color swatch with label
                let swatch_color = layer.color.fill.unwrap_or(layer.color.stroke);
                let egui_color = bevy_color_to_egui_color(swatch_color);
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::Vec2::splat(12.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, egui_color);
                    if let Some(geom_type) = layer.geom_type() {
                        ui.label(format!("Type: {}", geom_type));
                    } else {
                        ui.label("Type: Raster");
                    }
                });

                ui.separator();

                let manage_btn = ui.button("Manage...");
                crate::widget_registry::register("Manage", manage_btn.rect);
                if manage_btn.clicked() {
                    actions.push(SidePanelAction::ShowManageLayerWindow(layer.id));
                }

                let zoom_btn = ui.button("Zoom to Extent");
                crate::widget_registry::register("Zoom to extent", zoom_btn.rect);
                if zoom_btn.clicked() {
                    actions.push(SidePanelAction::CenterCamera(layer.id));
                }

                ui.separator();

                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(is_move_up_enabled, egui::Button::new("Move Up"))
                        .clicked()
                    {
                        actions.push(SidePanelAction::MoveLayer(layer.id, MoveDirection::Up));
                    }

                    if ui
                        .add_enabled(is_move_down_enabled, egui::Button::new("Move Down"))
                        .clicked()
                    {
                        actions.push(SidePanelAction::MoveLayer(layer.id, MoveDirection::Down));
                    }
                });

                ui.separator();

                if layer.is_vector() {
                    let ops_header = egui::CollapsingHeader::new("Operations")
                        .id_salt(format!("{:?}-operations", layer.id))
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                                crate::widgets::operations::Operations { layer }
                                    .show(ui, actions);
                            });
                        });
                    crate::widget_registry::register(
                        "Operations",
                        ops_header.header_response.rect,
                    );
                }

                ui.separator();

                let remove_btn = ui.button("Remove");
                crate::widget_registry::register("Remove", remove_btn.rect);
                if remove_btn.clicked() {
                    actions.push(SidePanelAction::DeleteLayer(layer.id));
                }
            });

        crate::widget_registry::register(&layer.name, response.header_response.rect);
        response.header_response
    }
}
