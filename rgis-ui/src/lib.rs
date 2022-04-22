#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;
use bevy_egui::egui;

mod add_layer_window;
mod bottom_panel;
mod manage_layer_window;
mod message_window;
mod side_panel;
mod top_panel;

pub struct Plugin;

#[derive(Debug, Default)]
struct UiState {
    /// Is the 'manage layer' window visible?
    pub is_manage_layer_window_visible: bool,
    /// Which layer is the user currently managing.
    pub managing_layer: Option<rgis_layer_id::LayerId>,
    /// Is the 'add layer' window visible?
    pub is_add_layer_window_visible: bool,
    pub messages: Vec<String>,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UiState {
                is_manage_layer_window_visible: false,
                managing_layer: None,
                is_add_layer_window_visible: true,
                messages: vec![],
            })
            .add_system(handle_open_file_task)
            .add_plugin(rgis_task::TaskPlugin::<add_layer_window::OpenFileTask>::new())
            .add_system(render_message_window.label("message_window"))
            .add_system_set(
                SystemSet::new()
                    .label("top_bottom_panels")
                    .after("message_window")
                    .with_system(render_top_panel)
                    .with_system(render_bottom_panel),
            )
            .add_system(
                render_side_panel
                    .label("side_panel")
                    .after("top_bottom_panels"),
            )
            .add_system(
                render_manage_layer_window
                    .label("manage_layer_window")
                    .after("side_panel"),
            )
            .add_system(
                render_add_layer_window
                    .label("add_layer_window")
                    .after("side_panel"),
            )
            .add_system(render_in_progress.after("top_bottom_panels"));
    }
}

fn render_top_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut app_exit_events: ResMut<bevy::ecs::event::Events<bevy::app::AppExit>>,
    mut windows: ResMut<Windows>,
) {
    top_panel::TopPanel {
        bevy_egui_ctx: &mut bevy_egui_ctx,
        app_exit_events: &mut app_exit_events,
        windows: &mut windows,
    }
    .render();
}

fn render_bottom_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mouse_pos: Res<rgis_mouse::MousePos>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
) {
    bottom_panel::BottomPanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        mouse_pos: &mouse_pos,
        rgis_settings: &rgis_settings,
    }
    .render();
}

fn render_side_panel(
    mut state: ResMut<UiState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut events: side_panel::Events,
) {
    side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        state: &mut state,
        layers: &layers,
        events: &mut events,
    }
    .render();
}

fn handle_open_file_task(
    mut events: ResMut<
        bevy::ecs::event::Events<rgis_task::TaskFinishedEvent<add_layer_window::OpenFileTask>>,
    >,
    mut load_geo_json_file_events: bevy::ecs::event::EventWriter<rgis_events::LoadGeoJsonFileEvent>,
    mut state: ResMut<UiState>,
) {
    for event in events.drain() {
        if let Some(outcome) = event.outcome {
            load_geo_json_file_events.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name: outcome.0,
                bytes: outcome.1,
                crs: "EPSG:4326".into(),
            });
            state.is_add_layer_window_visible = false;
        }
    }
}

fn render_manage_layer_window(
    mut state: ResMut<UiState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut color_events: ResMut<bevy::ecs::event::Events<rgis_events::UpdateLayerColor>>,
) {
    manage_layer_window::ManageLayerWindow {
        state: &mut state,
        layers: &layers,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        color_events: &mut color_events,
    }
    .render();
}

fn render_add_layer_window(
    mut state: ResMut<UiState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut load_geo_json_file_event_writer: bevy::ecs::event::EventWriter<
        rgis_events::LoadGeoJsonFileEvent,
    >,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    mut commands: bevy::ecs::system::Commands,
) {
    add_layer_window::AddLayerWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        thread_pool: &thread_pool,
        load_geo_json_file_event_writer: &mut load_geo_json_file_event_writer,
        commands: &mut commands,
    }
    .render();
}

fn render_message_window(
    mut state: ResMut<UiState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    message_window::MessageWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
    }
    .render();
}

fn render_in_progress(
    query: Query<&rgis_task::InProgressTask>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    let mut running_tasks = vec![];
    for task in query.iter() {
        running_tasks.push(task.task_name.clone());
    }
    if !running_tasks.is_empty() {
        egui::Window::new("Running tasks")
            .open(&mut true)
            .title_bar(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, [-5., -5.])
            .show(bevy_egui_ctx.ctx_mut(), |ui| {
                for task_name in running_tasks {
                    ui.horizontal(|ui| {
                        ui.add(egui::Spinner::new());
                        ui.label(format!("Running '{}'", task_name));
                    });
                }
            });
    }
}
