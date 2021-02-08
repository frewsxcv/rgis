use bevy::prelude::*;
use bevy_egui::egui;

pub struct RgisUi {
    pub source_srs: String,
    pub target_srs: String,
}

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

impl Plugin for RgisUi {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(bevy_egui::EguiPlugin)
            .add_resource(UiState {
                latitude: 0.,
                longitude: 0.,
                layers: vec![],
                source_srs: self.source_srs.to_owned(),
                target_srs: self.target_srs.to_owned(),
            })
            .add_system(ui.system());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub latitude: f32,
    pub longitude: f32,
    pub layers: Vec<String>,
    pub source_srs: String,
    pub target_srs: String,
}

fn ui(mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>, ui_state: Res<UiState>) {
    render_mouse_position_window(&mut bevy_egui_ctx.ctx, &ui_state);
    render_layers_window(&mut bevy_egui_ctx.ctx, &ui_state);
}

fn render_mouse_position_window(ctx: &mut egui::CtxRef, ui_state: &UiState) {
    egui::Window::new("Mouse position").show(ctx, |ui| {
        ui.label(format!("Source SRS: {}", ui_state.target_srs));
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.label(format!("X: {}", ui_state.latitude));
            ui.label(format!("Y: {}", ui_state.longitude));
        });

        ui.label(format!("Target SRS: {}", ui_state.target_srs));
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.label(format!("X: {}", ui_state.latitude));
            ui.label(format!("Y: {}", ui_state.longitude));
        });
    });
}

fn render_layers_window(ctx: &mut egui::CtxRef, ui_state: &UiState) {
    egui::Window::new("Layers").show(ctx, |ui| {
        for layer in &ui_state.layers {
            ui.label(format!("- {}", layer));
        }
    });
}
