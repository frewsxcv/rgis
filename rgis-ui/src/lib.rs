use bevy::prelude::*;
use bevy_egui::egui;

pub struct RgisUi;

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
            })
            .add_system(ui.system());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub latitude: f32,
    pub longitude: f32,
    pub layers: Vec<String>,
}

fn ui(mut egui_context: ResMut<bevy_egui::EguiContext>, ui_state: Res<UiState>) {
    let ctx = &mut egui_context.ctx;

    egui::Window::new("Mouse position").show(ctx, |ui| {
        ui.label(format!("Lat: {}", ui_state.latitude));
        ui.label(format!("Lng: {}", ui_state.longitude));
    });

    egui::Window::new("Layers").show(ctx, |ui| {
        for layer in &ui_state.layers {
            ui.label(format!("- {}", layer));
        }
    });
}
