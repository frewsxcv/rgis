use bevy::prelude::*;

mod bottom_panel;
mod manage_layer_window;
mod side_panel;
mod top_panel;

pub struct RgisUi;

type OpenedFileBytes = Vec<u8>;
type OpenedFileName = String;
type OpenedFileBytesSender = async_channel::Sender<(OpenedFileName, OpenedFileBytes)>;
type OpenedFileBytesReceiver = async_channel::Receiver<(OpenedFileName, OpenedFileBytes)>;

#[derive(Debug, Default)]
struct UiState {
    /// If the layer window is visible.
    pub layer_window_visible: bool,
    /// Which layer is the user currently managing.
    pub managing_layer: Option<rgis_layer_id::LayerId>,
}

impl Plugin for RgisUi {
    fn build(&self, app: &mut App) {
        let (sender, receiver): (OpenedFileBytesSender, OpenedFileBytesReceiver) =
            async_channel::unbounded();
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(sender)
            .insert_resource(receiver)
            .insert_resource(UiState {
                layer_window_visible: false,
                managing_layer: None,
            })
            .add_system_set(
                SystemSet::new()
                    .label("top_bottom_panels")
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
            );
    }
}

fn render_top_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut app_exit_events: ResMut<bevy::app::Events<bevy::app::AppExit>>,
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
    mut toggle_events: ResMut<bevy::app::Events<rgis_events::ToggleLayerVisibilityEvent>>,
    mut toggle_material_events: ResMut<bevy::app::Events<rgis_events::ToggleMaterialEvent>>,
    mut center_layer_events: ResMut<bevy::app::Events<rgis_events::CenterCameraEvent>>,
    mut load_geo_json_file_events: ResMut<bevy::app::Events<rgis_events::LoadGeoJsonFileEvent>>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    opened_file_bytes_sender: Res<OpenedFileBytesSender>,
    opened_file_bytes_receiver: Res<OpenedFileBytesReceiver>,
) {
    side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        state: &mut state,
        layers: &layers,
        toggle_events: &mut toggle_events,
        toggle_material_events: &mut toggle_material_events,
        center_layer_events: &mut center_layer_events,
        thread_pool: &thread_pool,
        opened_file_bytes_sender: &opened_file_bytes_sender,
    }
    .render();

    while let Ok((file_name, bytes)) = opened_file_bytes_receiver.try_recv() {
        load_geo_json_file_events.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
            file_name,
            bytes,
            source_crs: "EPSG:4326".into(),
        });
    }
}

fn render_manage_layer_window(
    mut state: ResMut<UiState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut color_events: ResMut<bevy::app::Events<rgis_events::UpdateLayerColor>>,
) {
    manage_layer_window::ManageLayerWindow {
        state: &mut state,
        layers: &layers,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        color_events: &mut color_events,
    }
    .render();
}
