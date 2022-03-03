use bevy::prelude::*;

mod manage_layer_window;
mod side_panel;
mod top_panel;

pub struct RgisUi {
    pub source_srs: String,
    pub target_srs: String,
}

// A unit struct to help identify the Position Text component, since there may be many Text components
pub struct PositionText;

type OpenedFileBytes = Vec<u8>;
type OpenedFileName = String;
type OpenedFileBytesSender = async_channel::Sender<(OpenedFileName, OpenedFileBytes)>;
type OpenedFileBytesReceiver = async_channel::Receiver<(OpenedFileName, OpenedFileBytes)>;

impl Plugin for RgisUi {
    fn build(&self, app: &mut App) {
        let (sender, receiver): (OpenedFileBytesSender, OpenedFileBytesReceiver) =
            async_channel::unbounded();
        app.add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(sender)
            .insert_resource(receiver)
            .insert_resource(UiState {
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
    pub source_srs: String,
    pub target_srs: String,
    /// If the layer window is visible.
    pub layer_window_visible: bool,
    /// Which layer is the user currently managing.
    pub managing_layer: Option<rgis_layer_id::LayerId>,
}

fn ui(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut ui_state: ResMut<UiState>,
    rgis_layers_resource: Res<rgis_layers::ArcLayers>,
    mut toggle_events: ResMut<bevy::app::Events<rgis_events::ToggleLayerVisibilityEvent>>,
    mut toggle_material_events: ResMut<bevy::app::Events<rgis_events::ToggleMaterialEvent>>,
    mut center_layer_events: ResMut<bevy::app::Events<rgis_events::CenterCameraEvent>>,
    mut load_geo_json_file_events: ResMut<bevy::app::Events<rgis_events::LoadGeoJsonFileEvent>>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    opened_file_bytes_sender: Res<OpenedFileBytesSender>,
    opened_file_bytes_receiver: Res<OpenedFileBytesReceiver>,
    mut app_exit_events: ResMut<bevy::app::Events<bevy::app::AppExit>>,
    mut windows: ResMut<Windows>,
    mouse_pos: Res<rgis_mouse::MousePos>,
) {
    top_panel::TopPanel {
        bevy_egui_ctx: &mut bevy_egui_ctx,
        app_exit_events: &mut app_exit_events,
        windows: &mut windows,
    }
    .render();

    side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        ui_state: &mut ui_state,
        rgis_layers_resource: &rgis_layers_resource,
        toggle_events: &mut toggle_events,
        toggle_material_events: &mut toggle_material_events,
        center_layer_events: &mut center_layer_events,
        thread_pool: &thread_pool,
        load_geo_json_file_events: &mut load_geo_json_file_events,
        opened_file_bytes_sender: &opened_file_bytes_sender,
        mouse_pos: &mouse_pos,
    }
    .render();

    manage_layer_window::ManageLayerWindow {
        ui_state: &mut ui_state,
        rgis_layers_resource: &rgis_layers_resource,
        bevy_egui_ctx: &mut bevy_egui_ctx,
    }
    .render();

    while let Ok((file_name, bytes)) = opened_file_bytes_receiver.try_recv() {
        load_geo_json_file_events.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
            file_name,
            bytes,
            source_srs: "EPSG:4326".into(),
            target_srs: "EPSG:3857".into(),
        });
    }
}
