use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::windows::add_layer::file::{OpenFileJob, SelectedFile};

pub(super) fn render_manage_layer_window(
    mut state: Local<crate::ManageLayerWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_query: Query<(
        &rgis_layers::LayerName,
        &rgis_layers::LayerColor,
        &rgis_layers::LayerPointSize,
        &rgis_layers::LayerData,
        &rgis_layers::LayerCrs,
    )>,
    mut color_events: ResMut<Messages<rgis_ui_messages::UpdateLayerColorMessage>>,
    mut point_size_events: ResMut<Messages<rgis_ui_messages::UpdateLayerPointSizeMessage>>,
    mut show_manage_layer_window_event_reader: MessageReader<
        rgis_ui_messages::ShowManageLayerWindowMessage,
    >,
    mut duplicate_layer_events: ResMut<Messages<rgis_events::DuplicateLayerMessage>>,
    mut rename_events: ResMut<Messages<rgis_ui_messages::RenameLayerMessage>>,
    mut name_edit_buffer: Local<String>,
    mut name_edit_layer_id: Local<Option<rgis_primitives::LayerId>>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
) -> Result {
    if let Some(event) = show_manage_layer_window_event_reader.read().last() {
        *state = Some(event.0);
    }

    let Some(layer_id) = *state else {
        return Ok(());
    };

    let Some(entity) = id_map.get(layer_id) else {
        *state = None;
        return Ok(());
    };
    let Ok((name, color, point_size, data, crs)) = layer_query.get(entity) else {
        *state = None;
        return Ok(());
    };

    // Initialize or reset the edit buffer when switching layers
    if *name_edit_layer_id != Some(layer_id) {
        *name_edit_layer_id = Some(layer_id);
        name_edit_buffer.clone_from(&name.0);
    }

    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 4.0);
    let mut is_open = true;
    egui::Window::new("Manage Layer")
        .default_pos(default_pos)
        .open(&mut is_open)
        .show(bevy_egui_ctx_mut, |ui| {
            crate::windows::manage_layer::ManageLayer {
                layer_id,
                name,
                color,
                point_size,
                data,
                crs,
                color_events: &mut color_events,
                point_size_events: &mut point_size_events,
                duplicate_layer_events: &mut duplicate_layer_events,
                rename_events: &mut rename_events,
                name_edit_buffer: &mut name_edit_buffer,
            }
            .render(ui);
        });

    if !is_open {
        *state = None;
    }
    Ok(())
}

pub(super) fn render_add_layer_window(
    mut is_visible: Local<bool>,
    mut selected_file: ResMut<SelectedFile>,
    mut bevy_egui_ctx: EguiContexts,
    mut job_spawner: bevy_jobs::JobSpawner,
    mut state: Local<crate::windows::add_layer::State>,
    mut events: crate::windows::add_layer::Events,
    geodesy_ctx: Res<rgis_crs::GeodesyContext>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
) -> Result {
    if crate::widget_registry::take_close_request("Add Layer") {
        state.reset();
        *is_visible = false;
        return Ok(());
    }

    if !events.show_add_layer_window_event_reader.is_empty() {
        *is_visible = true;
    }

    if !events.hide_add_layer_window_events.is_empty() {
        state.reset();
        *is_visible = false;
    }

    let mut output = None;

    if *is_visible {
        let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
        let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 4.0);
        egui::Window::new("Add Layer")
            .resizable(false)
            .default_pos(default_pos)
            .open(&mut is_visible)
            .show(bevy_egui_ctx_mut, |ui| {
                output = crate::windows::add_layer::AddLayer {
                    state: &mut state,
                    selected_file: &mut selected_file,
                    geodesy_ctx: &geodesy_ctx,
                }
                .render(ui);
            });
    } else {
        state.reset();
    }

    if let Some(output) = output {
        use crate::windows::add_layer::AddLayerOutput;
        match output {
            AddLayerOutput::LoadFromText {
                text,
                file_format,
                source_crs,
            } => {
                events.load_file_event_writer.write(
                    rgis_events::LoadFileMessage::FromBytes {
                        file_name: "Inputted file".into(),
                        file_format,
                        bytes: text.into(),
                        source_crs,
                    },
                );
                events.hide_add_layer_window_events.write_default();
                state.reset();
            }
            AddLayerOutput::LoadFromFile {
                file_name,
                file_format,
                bytes,
                source_crs,
            } => {
                events.load_file_event_writer.write(
                    rgis_events::LoadFileMessage::FromBytes {
                        file_name,
                        file_format,
                        bytes: bytes.into(),
                        source_crs,
                    },
                );
                events.hide_add_layer_window_events.write_default();
                state.reset();
            }
            AddLayerOutput::LoadFromLibrary {
                name,
                url,
                source_crs,
            } => {
                events.load_file_event_writer.write(
                    rgis_events::LoadFileMessage::FromNetwork {
                        name,
                        url,
                        source_crs,
                    },
                );
                events.hide_add_layer_window_events.write_default();
                state.reset();
            }
            AddLayerOutput::OpenFile => {
                job_spawner.spawn(OpenFileJob);
            }
        }
    }

    Ok(())
}

pub(super) fn render_change_crs_window(
    mut is_visible: ResMut<crate::ChangeCrsWindowVisible>,
    target_crs: Res<rgis_crs::TargetCrs>,
    mut bevy_egui_ctx: EguiContexts,
    mut text_field_value: Local<String>,
    mut crs_input_mode: Local<crate::widgets::crs_input::CrsInputMode>,
    mut change_crs_event_writer: MessageWriter<rgis_events::ChangeCrsMessage>,
    mut crs_input_outcome: Local<Option<crate::widgets::crs_input::Outcome>>,
    geodesy_ctx: Res<rgis_crs::GeodesyContext>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
    mut was_visible: Local<bool>,
) -> Result {
    if crate::widget_registry::take_close_request("Change CRS") {
        is_visible.0 = false;
        return Ok(());
    }

    let just_opened = is_visible.0 && !*was_visible;
    *was_visible = is_visible.0;

    if !is_visible.0 {
        return Ok(());
    }

    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 4.0);
    egui::Window::new("Change CRS")
        .default_pos(default_pos)
        .open(&mut is_visible.0)
        .show(bevy_egui_ctx_mut, |ui| {
            crate::windows::change_crs::ChangeCrs {
                text_field_value: &mut text_field_value,
                crs_input_mode: &mut crs_input_mode,
                change_crs_event_writer: &mut change_crs_event_writer,
                target_crs: (*target_crs).clone(),
                crs_input_outcome: &mut crs_input_outcome,
                geodesy_ctx: &geodesy_ctx,
                request_focus: just_opened,
            }
            .render(ui);
        });
    Ok(())
}

pub(super) fn render_feature_properties_window(
    mut state: Local<crate::FeaturePropertiesWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_name_query: Query<&rgis_layers::LayerName>,
    mut render_message_events: ResMut<Messages<rgis_ui_messages::RenderFeaturePropertiesMessage>>,
    mut features_deselected_writer: MessageWriter<rgis_events::FeaturesDeselectedMessage>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
) -> Result {
    if let Some(event) = render_message_events.drain().last() {
        if let Some(properties) = event.properties {
            *state = Some(crate::FeaturePropertiesWindowData {
                layer_id: event.layer_id,
                properties,
            });
        }
    }

    let Some(ref data) = *state else {
        return Ok(());
    };

    let Some(entity) = id_map.get(data.layer_id) else {
        return Ok(());
    };
    let Ok(layer_name) = layer_name_query.get(entity) else {
        return Ok(());
    };

    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 4.0);
    let properties = &data.properties;
    let mut is_open = true;
    egui::Window::new("Layer Feature Properties")
        .id(egui::Id::new("Layer Feature Properties Window"))
        .default_pos(default_pos)
        .open(&mut is_open)
        .show(bevy_egui_ctx_mut, |ui| {
            crate::windows::feature_properties::FeatureProperties {
                layer_name: &layer_name.0,
                properties,
            }
            .render(ui);
        });

    if !is_open {
        *state = None;
        features_deselected_writer.write(rgis_events::FeaturesDeselectedMessage);
    }
    Ok(())
}

pub(super) fn render_attribute_table_window(
    mut state: Local<crate::AttributeTableWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_query: Query<(&rgis_layers::LayerName, &rgis_layers::LayerData)>,
    mut show_events: MessageReader<rgis_ui_messages::ShowAttributeTableMessage>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
    mut center_camera_on_feature_writer: MessageWriter<rgis_events::CenterCameraOnFeatureMessage>,
    mut feature_selected_writer: MessageWriter<rgis_events::FeatureSelectedMessage>,
) -> Result {
    if let Some(event) = show_events.read().last() {
        *state = Some(event.0);
    }

    let Some(layer_id) = *state else {
        return Ok(());
    };

    let Some(entity) = id_map.get(layer_id) else {
        *state = None;
        return Ok(());
    };
    let Ok((name, data)) = layer_query.get(entity) else {
        *state = None;
        return Ok(());
    };

    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 40.0);
    let mut is_open = true;
    let mut action = None;
    egui::Window::new(format!("Attribute Table: {}", name.0))
        .id(egui::Id::new("Attribute Table Window"))
        .default_pos(default_pos)
        .default_size([600.0, 400.0])
        .resizable(true)
        .open(&mut is_open)
        .show(bevy_egui_ctx_mut, |ui| {
            action = crate::windows::attribute_table::AttributeTable { data }.render(ui);
        });

    match action {
        Some(crate::windows::attribute_table::AttributeTableAction::ZoomToFeature(feature_id)) => {
            center_camera_on_feature_writer.write(rgis_events::CenterCameraOnFeatureMessage(layer_id, feature_id));
        }
        Some(crate::windows::attribute_table::AttributeTableAction::SelectFeature(feature_id)) => {
            feature_selected_writer.write(rgis_events::FeatureSelectedMessage(layer_id, feature_id));
        }
        None => {}
    }

    if !is_open {
        *state = None;
    }
    Ok(())
}

pub(super) fn render_message_window(
    mut state: Local<crate::MessageWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    mut render_message_events: ResMut<Messages<rgis_ui_messages::RenderTextMessage>>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    if let Some(event) = render_message_events.drain().last() {
        *state = Some(event.0);
    }

    crate::windows::message::Message {
        state: &mut state,
        egui_ctx: bevy_egui_ctx_mut,
    }
    .render();
    Ok(())
}

pub(super) fn render_operation_window(
    mut state: Local<crate::OperationWindowState>,
    mut events: ResMut<Messages<rgis_ui_messages::OpenOperationWindowMessage>>,
    mut bevy_egui_ctx: EguiContexts,
    create_layer_event_writer: MessageWriter<rgis_events::CreateLayerMessage>,
    render_message_event_writer: MessageWriter<rgis_ui_messages::RenderTextMessage>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    if let Some(event) = events.drain().last() {
        *state = Some(crate::OperationWindowData {
            operation: event.operation,
            feature_collection: event.feature_collection,
            source_crs: None,
            layer_name: event.layer_name,
        });
    }

    if state.is_some() {
        let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 4.0);
        let mut is_open = true;
        egui::Window::new("Operation")
            .default_pos(default_pos)
            .open(&mut is_open)
            .show(bevy_egui_ctx_mut, |ui| {
                crate::windows::operation::Operation {
                    state: &mut state,
                    create_layer_event_writer,
                    render_message_event_writer,
                }
                .render(ui);
            });
        if !is_open {
            *state = None;
        }
    }
    Ok(())
}
