use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui::{self},
    EguiContexts, EguiTextureHandle,
};

pub(super) fn render_bottom(
    mut bevy_egui_ctx: EguiContexts,
    mouse_pos: Res<rgis_mouse::MousePos>,
    target_crs: Res<rgis_crs::TargetCrs>,
    geodesy_ctx: Res<rgis_crs::GeodesyContext>,
    wgs84_op_handle: Res<rgis_crs::Wgs84OpHandle>,
    mut change_crs_window_visible: ResMut<crate::ChangeCrsWindowVisible>,
    mut bottom_panel_height: ResMut<rgis_units::BottomPanelHeight>,
    mut cached_latlng: Local<Option<(f64, f64)>>,
) -> Result {
    // Only recompute the coordinate transformation when mouse position or CRS changes
    if mouse_pos.is_changed() || target_crs.is_changed() {
        *cached_latlng = crate::panels::bottom::projected_to_latlng(
            &mouse_pos,
            &target_crs,
            &geodesy_ctx,
            &wgs84_op_handle,
        );
    }
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    crate::panels::bottom::Bottom {
        egui_ctx: bevy_egui_ctx_mut,
        mouse_pos: &mouse_pos,
        target_crs: &target_crs,
        cached_latlng: *cached_latlng,
        change_crs_window_visible: &mut change_crs_window_visible,
        bottom_panel_height: &mut bottom_panel_height,
    }
    .render();
    Ok(())
}

pub(super) fn render_side(
    mut bevy_egui_ctx: EguiContexts,
    layer_order: Res<rgis_layers::LayerOrder>,
    layer_query: Query<(
        Entity,
        &rgis_primitives::LayerId,
        &rgis_layers::LayerName,
        &rgis_layers::LayerVisible,
        &rgis_layers::LayerColor,
        &rgis_layers::LayerPointSize,
        &rgis_layers::LayerData,
        &rgis_layers::LayerCrs,
    )>,
    mut events: crate::panels::side::Events,
    mut side_panel_width: ResMut<rgis_units::SidePanelWidth>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;

    // Build snapshots in top-to-bottom order to avoid passing Query across lifetime boundaries
    let snapshots: Vec<crate::panels::side::LayerSnapshot> = layer_order
        .iter_top_to_bottom()
        .filter_map(|entity| {
            let (_entity, layer_id, name, visible, color, _point_size, data, crs) =
                layer_query.get(entity).ok()?;
            Some(crate::panels::side::LayerSnapshot {
                layer_id: *layer_id,
                name: name.0.clone(),
                visible: visible.0,
                color: color.clone(),
                is_vector: data.is_vector(),
                is_active: data.is_active(),
                geom_type: data.geom_type(),
                crs: crs.clone(),
                unprojected_fc: data.unprojected_feature_collection().cloned(),
            })
        })
        .collect();

    crate::panels::side::Side {
        egui_ctx: bevy_egui_ctx_mut,
        snapshots,
        events: &mut events,
        side_panel_width: &mut side_panel_width,
    }
    .render();
    Ok(())
}

#[derive(Default)]
pub(super) struct LogoTextures {
    light: Option<(Handle<Image>, egui::TextureId)>,
    dark: Option<(Handle<Image>, egui::TextureId)>,
}

pub(super) fn render_top(
    mut bevy_egui_ctx: EguiContexts,
    mut app_exit_events: ResMut<Messages<AppExit>>,
    mut windows: Query<&mut bevy::window::Window, With<PrimaryWindow>>,
    app_settings: Res<rgis_settings::RgisSettings>,
    current_tool: Res<State<rgis_settings::Tool>>,
    mut next_tool: ResMut<NextState<rgis_settings::Tool>>,
    mut top_panel_height: ResMut<rgis_units::TopPanelHeight>,
    mut next_logs_visibility: ResMut<
        NextState<bevy_egui_window::WindowVisibility<crate::windows::logs::Logs<'static>>>,
    >,
    mut show_add_layer_window_event_writer: MessageWriter<rgis_ui_messages::ShowAddLayerWindowMessage>,
    asset_server: Res<AssetServer>,
    mut logo_textures: Local<LogoTextures>,
    mut settings_to_apply: ResMut<SettingsToApply>,
) -> Result {
    if logo_textures.light.is_none() {
        let handle: Handle<Image> = asset_server.load("logo-black.png");
        let texture_id = bevy_egui_ctx.add_image(EguiTextureHandle::Strong(handle.clone()));
        logo_textures.light = Some((handle, texture_id));
    }
    if logo_textures.dark.is_none() {
        let handle: Handle<Image> = asset_server.load("logo-white.png");
        let texture_id = bevy_egui_ctx.add_image(EguiTextureHandle::Strong(handle.clone()));
        logo_textures.dark = Some((handle, texture_id));
    }

    let logo_texture_id = if app_settings.dark_mode {
        logo_textures.dark.as_ref()
    } else {
        logo_textures.light.as_ref()
    }
    .filter(|(handle, _)| asset_server.is_loaded_with_dependencies(handle))
    .map(|(_, id)| *id);

    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let Ok(mut window) = windows.single_mut() else {
        return Ok(());
    };

    let output = crate::panels::top::Top {
        egui_ctx: bevy_egui_ctx_mut,
        app_exit_events: &mut app_exit_events,
        window: &mut window,
        app_settings: &app_settings,
        current_tool: current_tool.get(),
        next_tool: &mut next_tool,
        top_panel_height: &mut top_panel_height,
        next_logs_visibility: &mut next_logs_visibility,
        show_add_layer_window_event_writer: &mut show_add_layer_window_event_writer,
        logo_texture_id,
    }
    .render();

    // Defer settings mutations so that RgisSettings change detection works correctly.
    // The actual mutation happens in apply_deferred_settings, which runs after render_top.
    if output.toggle_show_scale {
        settings_to_apply.toggle_show_scale = true;
    }
    if output.toggle_dark_mode {
        settings_to_apply.toggle_dark_mode = true;
    }

    Ok(())
}

/// Resource that buffers settings mutations from the top panel UI.
/// This allows `render_top` to use `Res<RgisSettings>` (immutable) so that
/// Bevy's change detection on `RgisSettings` fires only when settings actually
/// change, not every frame.
#[derive(Resource, Default)]
pub(super) struct SettingsToApply {
    pub toggle_show_scale: bool,
    pub toggle_dark_mode: bool,
}

/// Applies deferred settings mutations from the top panel, then resets the buffer.
pub(super) fn apply_deferred_settings(
    mut settings_to_apply: ResMut<SettingsToApply>,
    mut app_settings: ResMut<rgis_settings::RgisSettings>,
) {
    if !settings_to_apply.toggle_show_scale && !settings_to_apply.toggle_dark_mode {
        return;
    }
    if settings_to_apply.toggle_show_scale {
        app_settings.show_scale = !app_settings.show_scale;
    }
    if settings_to_apply.toggle_dark_mode {
        app_settings.dark_mode = !app_settings.dark_mode;
    }
    settings_to_apply.toggle_show_scale = false;
    settings_to_apply.toggle_dark_mode = false;
}

/// Synchronizes the egui theme and clear color when `RgisSettings` changes.
/// Thanks to the deferred-mutation pattern in `render_top` / `apply_deferred_settings`,
/// `RgisSettings` is only marked as changed when a setting is actually toggled,
/// so this system skips work on most frames.
pub(super) fn sync_egui_theme(
    mut bevy_egui_ctx: EguiContexts,
    mut clear_color: ResMut<ClearColor>,
    app_settings: Res<rgis_settings::RgisSettings>,
) -> Result {
    if !app_settings.is_changed() {
        return Ok(());
    }
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let dark_mode = app_settings.dark_mode;
    let egui_visuals = if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    // Set the background color of the map
    clear_color.0 = egui_color_to_bevy_color(egui_visuals.extreme_bg_color);
    // Set the egui theme
    bevy_egui_ctx_mut.set_visuals(egui_visuals);
    Ok(())
}

pub fn egui_color_to_bevy_color(egui_color: bevy_egui::egui::Color32) -> Color {
    Color::srgb_u8(egui_color.r(), egui_color.g(), egui_color.b())
}
