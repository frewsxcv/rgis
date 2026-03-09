mod event_handlers;
mod measure;
mod panels;
mod windows;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use bevy_egui_window::Window;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum RenderSystemSet {
    RenderingMessageWindow,
    RenderingTopBottom,
    Side,
    Windows,
}

pub fn configure(app: &mut App) {
    app.init_resource::<panels::SettingsToApply>();

    app.add_systems(
        PostStartup,
        (bevy_egui::setup_primary_egui_context_system, panels::sync_egui_theme).chain(),
    );

    app.configure_sets(
        EguiPrimaryContextPass,
        (
            RenderSystemSet::RenderingMessageWindow,
            RenderSystemSet::RenderingTopBottom,
            RenderSystemSet::Side,
            RenderSystemSet::Windows,
        )
            .chain(),
    );

    app.add_systems(
        EguiPrimaryContextPass,
        (
            crate::widgets::scale_bar::render_map_scale.in_set(RenderSystemSet::Side),
            crate::widgets::zoom_buttons::render_zoom_buttons.in_set(RenderSystemSet::Side),
            windows::render_message_window.in_set(RenderSystemSet::RenderingMessageWindow),
            panels::render_top.in_set(RenderSystemSet::RenderingTopBottom),
            panels::render_bottom.in_set(RenderSystemSet::RenderingTopBottom),
            panels::render_side.in_set(RenderSystemSet::Side),
            measure::render_in_progress.in_set(RenderSystemSet::Side),
            windows::render_manage_layer_window.in_set(RenderSystemSet::Windows),
            windows::render_add_layer_window.in_set(RenderSystemSet::Windows),
            windows::render_change_crs_window.in_set(RenderSystemSet::Windows),
            windows::render_feature_properties_window.in_set(RenderSystemSet::Windows),
            windows::render_attribute_table_window.in_set(RenderSystemSet::Windows),
            windows::render_operation_window.in_set(RenderSystemSet::Windows),
            measure::render_measure_tool.in_set(RenderSystemSet::RenderingTopBottom),
        ),
    );

    app.add_systems(
        Update,
        (
            event_handlers::handle_open_file_job,
            event_handlers::handle_save_file_job,
            event_handlers::handle_download_layer,
            event_handlers::perform_operation,
            event_handlers::handle_fill_color_requests,
            // Apply deferred settings mutations, then sync theme only when
            // RgisSettings actually changed.
            (panels::apply_deferred_settings, panels::sync_egui_theme).chain(),
        ),
    );

    crate::windows::logs::Logs::setup(app);
    crate::windows::welcome::Welcome::setup(app);
    app.add_systems(
        EguiPrimaryContextPass,
        (
            bevy_egui_window::render_window_system::<crate::windows::logs::Logs>
                .run_if(in_state(bevy_egui_window::WindowVisibility::<crate::windows::logs::Logs>::Open)),
            crate::windows::welcome::render_welcome_window_system
                .run_if(in_state(bevy_egui_window::WindowVisibility::<crate::windows::welcome::Welcome>::Open)),
        ),
    );
}
