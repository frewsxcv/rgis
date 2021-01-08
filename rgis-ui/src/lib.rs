use bevy::prelude::*;
use bevy_megaui::{megaui::{self, hash}, self};

pub struct RgisUi;

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

impl Plugin for RgisUi {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(bevy_megaui::MegaUiPlugin)
            .add_resource(UiState {
                latitude: 0.,
                longitude: 0.,
                layers: vec![],
            })
            .add_system(ui_example.system());
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

fn ui_example(_world: &mut World, resources: &mut Resources) {
    let mut mega_ui_context = resources.get_thread_local_mut::<bevy_megaui::MegaUiContext>().unwrap();
    let ui_state = resources.get::<UiState>().unwrap();

    mega_ui_context.draw_window(
        hash!(),
        megaui::Vector2::new(MOUSE_POSITION_WINDOW_OFFSET_X, MOUSE_POSITION_WINDOW_OFFSET_Y),
        megaui::Vector2::new(MOUSE_POSITION_WINDOW_WIDTH, MOUSE_POSITION_WINDOW_HEIGHT),
        bevy_megaui::WindowParams {
            label: String::from("Mouse position"),
            ..Default::default()
        },
        |ui| {
            ui.label(None, &format!("Lng: {}", ui_state.longitude));
            ui.label(None, &format!("Lat: {}", ui_state.latitude));
        },
    );

    mega_ui_context.draw_window(
        hash!(),
        megaui::Vector2::new(LAYERS_WINDOW_OFFSET_X, LAYERS_WINDOW_OFFSET_Y),
        megaui::Vector2::new(LAYERS_WINDOW_WIDTH, LAYERS_WINDOW_HEIGHT),
        bevy_megaui::WindowParams {
            label: String::from("Layers"),
            ..Default::default()
        },
        |ui| {
            for layer in &ui_state.layers {
                ui.label(None, &format!("- {}", layer));
            }
        },
    );
}
