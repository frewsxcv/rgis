use bevy::prelude::*;
use bevy_egui::egui;

const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

pub struct RgisUi {
    pub source_srs: String,
    pub target_srs: String,
}

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

impl Plugin for RgisUi {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UiState {
                projected_mouse_position: geo_srs::CoordWithSrs {
                    srs: self.target_srs.clone(),
                    coord: geo_types::Coordinate { x: 0., y: 0. },
                },
                source_srs: self.source_srs.to_owned(),
                target_srs: self.target_srs.to_owned(),
                layer_window_visible: false,
                managing_layer: None,
            })
            .add_system(ui.system());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub projected_mouse_position: geo_srs::CoordWithSrs<f32>,
    pub source_srs: String,
    pub target_srs: String,
    /// If the layer window is visible.
    pub layer_window_visible: bool,
    /// Which layer is the user currently managing.
    pub managing_layer: Option<rgis_layers::LayerId>,
}

fn ui(
    bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut ui_state: ResMut<UiState>,
    rgis_layers_resource: Res<rgis_layers::RgisLayersResource>,
) {
    render_side_panel(bevy_egui_ctx.ctx(), &mut ui_state, &rgis_layers_resource);

    match (ui_state.layer_window_visible, ui_state.managing_layer) {
        (true, Some(layer_id)) => {
            let layers = rgis_layers_resource.read().unwrap(); // TODO: remove unwrap
            let layer = layers.get(layer_id).unwrap(); // TOOD: remove unwrap
            egui::Window::new("Manage Layer")
                .open(&mut ui_state.layer_window_visible)
                .show(bevy_egui_ctx.ctx(), |ui| {
                    egui::Grid::new("FIXME").show(ui, |ui| {
                        ui.label("Name");
                        ui.label(layer.name.clone());
                        ui.end_row();
                    });
                });
        }
        _ => (),
    }
}

fn render_side_panel(
    ctx: &egui::CtxRef,
    ui_state: &mut UiState,
    rgis_layers_resource: &rgis_layers::RgisLayersResource,
) {
    egui::SidePanel::left("left-side-panel")
        .max_width(MAX_SIDE_PANEL_WIDTH)
        .show(ctx, |ui| {
            render_mouse_position_window(ui, ui_state);
            render_layers_window(ui, ui_state, rgis_layers_resource);
        });
}

fn render_mouse_position_window(ui: &mut egui::Ui, ui_state: &UiState) {
    ui.collapsing("Mouse Position", |ui| {
        let mut unprojected = ui_state.projected_mouse_position.clone();
        unprojected.reproject(&ui_state.source_srs);

        ui.label(format!("Source SRS: {}", ui_state.source_srs));
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(format!("X: {}", unprojected.coord.x));
            ui.label(format!("Y: {}", unprojected.coord.y));
        });

        ui.label(format!("Target SRS: {}", ui_state.target_srs));
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(format!("X: {}", ui_state.projected_mouse_position.coord.x));
            ui.label(format!("Y: {}", ui_state.projected_mouse_position.coord.y));
        });
    });
}

fn render_layers_window(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    rgis_layers_resource: &rgis_layers::RgisLayersResource,
) {
    ui.collapsing("Layers", |ui| {
        let rgis_layers_resource = match rgis_layers_resource.read() {
            Ok(r) => r,
            Err(_) => {
                // TODO log failure
                return;
            }
        };
        for layer in &rgis_layers_resource.data {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(layer.name.to_string());
                if ui.button("Manage").clicked() {
                    ui_state.layer_window_visible = true;
                    ui_state.managing_layer = Some(layer.id);
                }
            });
        }
    });
}
