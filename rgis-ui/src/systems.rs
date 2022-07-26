use bevy::prelude::*;
use bevy_egui::egui;

fn handle_render_feature_properties_event(
    mut render_message_events: ResMut<
        bevy::ecs::event::Events<rgis_events::RenderFeaturePropertiesEvent>,
    >,
    mut state: ResMut<crate::FeaturePropertiesWindowState>,
) {
    if let Some(event) = render_message_events.drain().last() {
        state.is_visible = true;
        state.properties = Some(event.0);
    }
}

fn handle_render_message_event(
    mut render_message_events: ResMut<bevy::ecs::event::Events<rgis_events::RenderMessageEvent>>,
    mut state: ResMut<crate::MessageWindowState>,
) {
    if let Some(event) = render_message_events.drain().last() {
        state.message = Some(event.0);
        state.is_visible = true;
    }
}

fn render_bottom_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mouse_pos: Res<rgis_mouse::MousePos>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mut open_change_crs_window_event_writer: bevy::ecs::event::EventWriter<
        rgis_events::OpenChangeCrsWindow,
    >,
    mut bottom_panel_height: ResMut<crate::BottomPanelHeight>,
) {
    crate::bottom_panel::BottomPanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        mouse_pos: &mouse_pos,
        rgis_settings: &rgis_settings,
        open_change_crs_window_event_writer: &mut open_change_crs_window_event_writer,
        bottom_panel_height: &mut bottom_panel_height,
    }
    .render();
}

fn render_side_panel(
    mut state: ResMut<crate::AddLayerWindowState>,
    mut manage_layer_window_state: ResMut<crate::ManageLayerWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut events: crate::side_panel::Events,
    mut side_panel_width: ResMut<crate::SidePanelWidth>,
) {
    crate::side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        state: &mut state,
        manage_layer_window_state: &mut manage_layer_window_state,
        layers: &layers,
        events: &mut events,
        side_panel_width: &mut side_panel_width,
    }
    .render();
}

fn handle_open_file_task(
    mut finished_tasks: bevy_jobs::FinishedJobs,
    mut load_geo_json_file_events: bevy::ecs::event::EventWriter<rgis_events::LoadGeoJsonFileEvent>,
    mut state: ResMut<crate::AddLayerWindowState>, // TODO: change this to Local?
) {
    while let Some(outcome) = finished_tasks.take_next::<crate::add_layer_window::OpenFileTask>() {
        if let Some(outcome) = outcome {
            load_geo_json_file_events.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name: outcome.0,
                bytes: outcome.1,
                crs: "EPSG:4326".into(),
            });
            state.is_visible = false;
        }
    }
}

fn render_manage_layer_window(
    mut state: ResMut<crate::ManageLayerWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut color_events: ResMut<bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>>,
) {
    crate::manage_layer_window::ManageLayerWindow {
        state: &mut state,
        layers: &layers,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        color_events: &mut color_events,
    }
    .render();
}

fn render_add_layer_window(
    mut state: ResMut<crate::AddLayerWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut load_geo_json_file_event_writer: bevy::ecs::event::EventWriter<
        rgis_events::LoadGeoJsonFileEvent,
    >,
    mut task_spawner: bevy_jobs::JobSpawner,
) {
    crate::add_layer_window::AddLayerWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        task_spawner: &mut task_spawner,
        load_geo_json_file_event_writer: &mut load_geo_json_file_event_writer,
    }
    .render();
}

fn render_change_crs_window(
    mut is_visible: Local<bool>,
    mut open_change_crs_window_event_reader: bevy::ecs::event::EventReader<
        rgis_events::OpenChangeCrsWindow,
    >,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut text_field_value: Local<String>,
    mut change_crs_event_writer: bevy::ecs::event::EventWriter<rgis_events::ChangeCrsEvent>,
) {
    if open_change_crs_window_event_reader.iter().next().is_some() {
        *is_visible = true;
    }

    crate::change_crs_window::ChangeCrsWindow {
        is_visible: &mut is_visible,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        text_field_value: &mut text_field_value,
        change_crs_event_writer: &mut change_crs_event_writer,
    }
    .render();
}

fn render_feature_properties_window(
    mut state: ResMut<crate::FeaturePropertiesWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    crate::feature_properties_window::FeaturePropertiesWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
    }
    .render();
}

fn render_message_window(
    mut state: ResMut<crate::MessageWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    crate::message_window::MessageWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
    }
    .render();
}

fn render_in_progress(
    query: Query<&bevy_jobs::InProgressJob>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    let mut task_name_iter = query.iter().map(|task| &task.name).peekable();

    if task_name_iter.peek().is_none() {
        return;
    }

    egui::Window::new("Running tasks")
        .open(&mut true)
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5., -5.])
        .show(bevy_egui_ctx.ctx_mut(), |ui| {
            for task_name in task_name_iter {
                ui.horizontal(|ui| {
                    ui.add(egui::Spinner::new());
                    ui.label(format!("Running '{}'", task_name));
                });
            }
        });
}

fn render_top_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut app_exit_events: ResMut<bevy::ecs::event::Events<bevy::app::AppExit>>,
    mut windows: ResMut<Windows>,
    mut app_settings: ResMut<rgis_settings::RgisSettings>,
    mut top_panel_height: ResMut<crate::TopPanelHeight>,
) {
    crate::top_panel::TopPanel {
        bevy_egui_ctx: &mut bevy_egui_ctx,
        app_exit_events: &mut app_exit_events,
        windows: &mut windows,
        app_settings: &mut app_settings,
        top_panel_height: &mut top_panel_height,
    }
    .render();
}

pub fn system_sets() -> [SystemSet; 2] {
    [
        SystemSet::new()
            .label("top_bottom_panels")
            .after("message_window")
            .with_system(render_top_panel)
            .with_system(render_bottom_panel),
        SystemSet::new()
            .with_system(handle_open_file_task)
            .with_system(handle_render_feature_properties_event)
            .with_system(handle_render_message_event)
            .with_system(render_message_window.label("message_window"))
            .with_system(
                render_side_panel
                    .label("side_panel")
                    .after("top_bottom_panels"),
            )
            .with_system(
                render_manage_layer_window
                    .label("manage_layer_window")
                    .after("side_panel"),
            )
            .with_system(
                render_add_layer_window
                    .label("add_layer_window")
                    .after("manage_layer_window"),
            )
            .with_system(
                render_change_crs_window
                    .label("change_crs_window")
                    .after("add_layer_window"),
            )
            .with_system(
                render_feature_properties_window
                    .label("feature_properties_window")
                    .after("add_layer_window"),
            )
            .with_system(render_in_progress.after("top_bottom_panels")),
    ]
}
