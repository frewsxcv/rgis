use bevy::ecs::event::Events;
use bevy::prelude::*;

fn load_geojson_file_handler(
    mut load_event_reader: ResMut<Events<rgis_events::LoadGeoJsonFileEvent>>,
    mut task_spawner: bevy_jobs::JobSpawner,
    mut finished_tasks: bevy_jobs::FinishedJobs,
) {
    while let Some(outcome) = finished_tasks.take_next::<rgis_network::NetworkFetchTask>() {
        match outcome {
            Ok(fetched) => load_event_reader.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
                bytes: fetched.bytes,
                file_name: fetched.name,
                crs: fetched.crs,
            }),
            Err(e) => {
                bevy::log::error!("Could not fetch file: {:?}", e);
            }
        }
    }

    for event in load_event_reader.drain() {
        match event {
            rgis_events::LoadGeoJsonFileEvent::FromNetwork { url, crs, name } => {
                task_spawner.spawn(rgis_network::NetworkFetchTask { url, crs, name })
            }
            rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name,
                bytes,
                crs,
            } => task_spawner.spawn(crate::tasks::LoadFileJob {
                file_loader: crate::geojson::GeoJsonSource { bytes },
                source_crs: crs,
                name: file_name,
            }),
        }
    }
}

fn handle_load_geojson_file_task_finished_events(
    mut finished_tasks: bevy_jobs::FinishedJobs,
    mut create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
) {
    while let Some(outcome) =
        finished_tasks.take_next::<crate::tasks::LoadFileJob<crate::geojson::GeoJsonSource>>()
    {
        match outcome {
            Ok(outcome) => create_layer_event_writer.send(rgis_events::CreateLayerEvent {
                name: outcome.name,
                unprojected_geometry: outcome.geometry,
                source_crs: outcome.source_crs,
            }),
            Err(e) => {
                bevy::log::error!("Encountered error when loading GeoJSON file: {:?}", e);
            }
        }
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(load_geojson_file_handler)
        .with_system(handle_load_geojson_file_task_finished_events)
}
