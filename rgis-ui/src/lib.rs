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
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .add_resource(UiState {
                layers: vec![],
                projected_mouse_position: geo_srs::CoordWithSrs {
                    srs: self.target_srs.clone(),
                    coord: geo_types::Coordinate { x: 0., y: 0. },
                },
                source_srs: self.source_srs.to_owned(),
                target_srs: self.target_srs.to_owned(),
            })
            .add_system(ui.system());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub projected_mouse_position: geo_srs::CoordWithSrs<f32>,
    pub layers: Vec<String>,
    pub source_srs: String,
    pub target_srs: String,
}

fn ui(mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>, ui_state: Res<UiState>) {
    render_side_panel(&mut bevy_egui_ctx.ctx, &ui_state);
}

fn render_side_panel(ctx: &mut egui::CtxRef, ui_state: &UiState) {
    egui::SidePanel::left("left-side-panel", MAX_SIDE_PANEL_WIDTH).show(ctx, |mut ui| {
        render_mouse_position_window(&mut ui, &ui_state);
        render_layers_window(&mut ui, &ui_state);
    });
}

fn render_mouse_position_window(ui: &mut egui::Ui, ui_state: &UiState) {
    ui.label("Mouse Position");

    egui::Frame::group(ui.style()).show(ui, |ui| {
        let mut unprojected = ui_state.projected_mouse_position.clone();
        unprojected.reproject(&ui_state.source_srs);

        ui.label(format!("Source SRS: {}", ui_state.target_srs));
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

fn render_layers_window(ui: &mut egui::Ui, ui_state: &UiState) {
    ui.label("Layers");

    egui::Frame::group(ui.style()).show(ui, |ui| {
        for layer in &ui_state.layers {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(layer);
                if ui.button("Manage").clicked() {
                    println!("Managed: {}", layer);
                }
            });
        }
    });
}
