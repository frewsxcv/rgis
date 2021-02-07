use bevy::prelude::*;
use bevy_egui::egui;

pub struct RgisUi;

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

impl Plugin for RgisUi {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .add_plugin(bevy_megaui::MegaUiPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .add_resource(UiState {
                latitude: 0.,
                longitude: 0.,
                layers: vec![],
            })
            .add_system(ui.system());
            // .add_system(ui_example.system());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub latitude: f32,
    pub longitude: f32,
    pub layers: Vec<String>,
}

const WINDOW_MARGIN_X: f32 = 5.;
const WINDOW_MARGIN_Y: f32 = 5.;
const WINDOW_WIDTH: f32 = 200.;

const MOUSE_POSITION_WINDOW_OFFSET_X: f32 = WINDOW_MARGIN_X;
const MOUSE_POSITION_WINDOW_OFFSET_Y: f32 = WINDOW_MARGIN_Y;
const MOUSE_POSITION_WINDOW_WIDTH: f32 = WINDOW_WIDTH;
const MOUSE_POSITION_WINDOW_HEIGHT: f32 = 55.;

const LAYERS_WINDOW_OFFSET_X: f32 = WINDOW_MARGIN_X;
const LAYERS_WINDOW_OFFSET_Y: f32 = MOUSE_POSITION_WINDOW_OFFSET_Y + MOUSE_POSITION_WINDOW_HEIGHT + WINDOW_MARGIN_Y;
const LAYERS_WINDOW_WIDTH: f32 = WINDOW_WIDTH;
const LAYERS_WINDOW_HEIGHT: f32 = 200.;

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
