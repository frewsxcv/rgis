use bevy::prelude::{App, Update};

fn handle_layer_created_events(
    mut layer_created_event_reader: bevy::ecs::event::EventReader<rgis_events::LayerCreatedEvent>,
    layers: bevy::ecs::system::Res<rgis_layers::Layers>,
    rgis_settings: bevy::ecs::system::Res<rgis_settings::RgisSettings>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    for event in layer_created_event_reader.read() {
        let Some(layer) = layers.get(event.0) else {
            continue;
        };

        job_spawner.spawn(crate::jobs::ReprojectGeometryJob {
            feature_collection: layer.unprojected_feature_collection.clone(),
            layer_id: event.0,
            source_epsg_code: layer.crs_epsg_code,
            target_epsg_code: rgis_settings.target_crs_epsg_code,
        });
    }
}

fn handle_reproject_geometry_job_completion_events(
    mut finished_jobs: bevy_jobs::FinishedJobs,
    mut layers: bevy::ecs::system::ResMut<rgis_layers::Layers>,
    mut job_spawner: bevy_jobs::JobSpawner,
    rgis_settings: bevy::ecs::system::Res<rgis_settings::RgisSettings>,
) {
    while let Some(outcome) = finished_jobs.take_next::<crate::jobs::ReprojectGeometryJob>() {
        let outcome = match outcome {
            Ok(o) => o,
            Err(e) => {
                bevy::log::error!("Encountered an error reprojecting geometry: {:?}", e);
                continue;
            }
        };

        if outcome.target_crs_epsg_code != rgis_settings.target_crs_epsg_code {
            bevy::log::error!("Encountered a reprojected geometry with a different CRS than the current target CRS");
            continue;
        }

        job_spawner.spawn(rgis_file_loader::LODBuilderJob {
            projected: outcome.feature_collection,
            layer_id: outcome.layer_id,
        });
    }
}

fn handle_crs_changed_events(
    mut crs_changed_event_reader: bevy::ecs::event::EventReader<rgis_events::CrsChangedEvent>,
    mut layers: bevy::ecs::system::ResMut<rgis_layers::Layers>,
    rgis_settings: bevy::ecs::system::Res<rgis_settings::RgisSettings>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    if crs_changed_event_reader.read().next().is_some() {
        layers.clear_projected();

        for layer in layers.iter() {
            job_spawner.spawn(crate::jobs::ReprojectGeometryJob {
                feature_collection: layer.unprojected_feature_collection.clone(),
                layer_id: layer.id,
                source_epsg_code: layer.crs_epsg_code,
                target_epsg_code: rgis_settings.target_crs_epsg_code,
            });
        }
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_layer_created_events,
            handle_reproject_geometry_job_completion_events,
            handle_crs_changed_events,
        ),
    );
}
