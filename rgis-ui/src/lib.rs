use bevy::prelude::*;
use bevy_egui::egui;

mod side_panel;
mod top_panel;

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
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut ui_state: ResMut<UiState>,
    rgis_layers_resource: Res<rgis_layers::RgisLayersResource>,
    mut toggle_events: ResMut<bevy::app::Events<rgis_layers::ToggleLayerVisibility>>,
    mut toggle_material_events: ResMut<bevy::app::Events<rgis_renderer::ToggleMaterialEvent>>,
    mut center_layer_events: ResMut<bevy::app::Events<rgis_renderer::CenterCameraEvent>>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
) {
    top_panel::TopPanel {
        bevy_egui_ctx: &mut bevy_egui_ctx,
        thread_pool: &thread_pool,
    }.render();

    side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        ui_state: &mut ui_state,
        rgis_layers_resource: &rgis_layers_resource,
        toggle_events: &mut toggle_events,
        toggle_material_events: &mut toggle_material_events,
        center_layer_events: &mut center_layer_events,
    }.render();

    match (ui_state.layer_window_visible, ui_state.managing_layer) {
        (true, Some(layer_id)) => {
            let layers = rgis_layers_resource.read().unwrap(); // TODO: remove unwrap
            let layer = layers.get(layer_id).unwrap(); // TOOD: remove unwrap
            egui::Window::new("Manage Layer")
                .open(&mut ui_state.layer_window_visible)
                .show(bevy_egui_ctx.ctx_mut(), |ui| {
                    egui::Grid::new("FIXME")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.label(layer.name.clone());
                            ui.end_row();
                        });
                });
        }
        _ => (),
    }
}
