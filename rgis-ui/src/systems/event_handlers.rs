use std::sync::Arc;

use bevy::prelude::*;

use crate::windows::add_layer::file::{OpenFileJob, SelectedFile};

pub(super) fn handle_open_file_job(
    mut finished_jobs: bevy_jobs::FinishedJobs,
    mut selected_file: ResMut<SelectedFile>,
) {
    while let Some(outcome) = finished_jobs.take_next::<OpenFileJob>().flatten() {
        selected_file.0 = Some(outcome);
    }
}

pub(super) fn handle_download_layer(
    mut events: MessageReader<rgis_events::DownloadLayerMessage>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_query: Query<(&rgis_layers::LayerName, &rgis_layers::LayerData)>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    for event in events.read() {
        let Some(entity) = id_map.get(event.layer_id) else {
            warn!("Could not find layer for download");
            continue;
        };
        let Ok((name, data)) = layer_query.get(entity) else {
            warn!("Could not find layer for download");
            continue;
        };

        let Some(fc) = data.unprojected_feature_collection() else {
            warn!("Cannot download raster layer");
            continue;
        };

        match rgis_layers::export::export_feature_collection(fc, event.format) {
            Ok(data) => {
                let default_name = format!("{}.{}", name.0, event.format.extension());
                job_spawner.spawn(crate::save_file::SaveFileJob {
                    data: data.into_bytes(),
                    default_name,
                    format: event.format,
                });
            }
            Err(e) => {
                error!("Failed to export layer: {}", e);
            }
        }
    }
}

pub(super) fn handle_save_file_job(mut finished_jobs: bevy_jobs::FinishedJobs) {
    while let Some(outcome) = finished_jobs.take_next::<crate::save_file::SaveFileJob>() {
        if let Err(e) = outcome {
            error!("Failed to save file: {}", e);
        }
    }
}

pub(super) fn handle_fill_color_requests(
    layer_order: Res<rgis_layers::LayerOrder>,
    layer_id_query: Query<&rgis_primitives::LayerId>,
    mut color_events: ResMut<Messages<rgis_ui_messages::UpdateLayerColorMessage>>,
) {
    for rgba in crate::widget_registry::take_fill_color_requests() {
        // Apply to the first layer
        if let Some(entity) = layer_order.iter_top_to_bottom().next() {
            if let Ok(layer_id) = layer_id_query.get(entity) {
                color_events.write(rgis_ui_messages::UpdateLayerColorMessage::Fill(
                    *layer_id,
                    Color::linear_rgba(rgba[0], rgba[1], rgba[2], rgba[3]),
                ));
            }
        }
    }
}

pub(super) fn perform_operation(
    mut events: ResMut<Messages<rgis_ui_messages::PerformOperationMessage>>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_query: Query<(&rgis_layers::LayerName, &rgis_layers::LayerData, &rgis_layers::LayerCrs)>,
    mut open_operation_window_event_writer: MessageWriter<rgis_ui_messages::OpenOperationWindowMessage>,
    mut create_layer_event_writer: MessageWriter<rgis_events::CreateLayerMessage>,
    mut render_message_event_writer: MessageWriter<rgis_ui_messages::RenderTextMessage>,
) {
    for event in events.drain() {
        let Some(entity) = id_map.get(event.layer_id) else {
            error!("Layer not found, cannot perform operation");
            continue;
        };
        let Ok((name, data, crs)) = layer_query.get(entity) else {
            error!("Layer not found, cannot perform operation");
            continue;
        };

        let Some(_) = data.unprojected_feature_collection() else {
            error!("Cannot perform operation on raster layer");
            continue;
        };

        let mut operation = event.operation;

        let Some(fc) = data.unprojected_feature_collection() else {
            error!("Layer has no unprojected feature collection, cannot perform operation");
            continue;
        };

        match operation.next_action() {
            rgis_geo_ops::Action::RenderUi => {
                open_operation_window_event_writer.write(
                    rgis_ui_messages::OpenOperationWindowMessage {
                        operation,
                        feature_collection: Arc::clone(fc),
                        layer_name: name.0.clone(),
                    },
                );
            }
            rgis_geo_ops::Action::Perform => {
                // TODO: perform in background job
                let op_name = operation.name().to_string();
                let outcome = operation.perform(fc);

                match outcome {
                    Ok(rgis_geo_ops::Outcome::FeatureCollection(feature_collection)) => {
                        create_layer_event_writer.write(rgis_events::CreateLayerMessage {
                            feature_collection: Arc::new(feature_collection),
                            name: format!("{} of {}", op_name, name.0),
                            source_crs: crs.0.clone(),
                        });
                    }
                    Ok(rgis_geo_ops::Outcome::Text(text)) => {
                        render_message_event_writer.write(rgis_ui_messages::RenderTextMessage(text));
                    }
                    Err(e) => {
                        error!("Encountered an error during the operation: {}", e);
                    }
                }
            }
        }
    }
}
