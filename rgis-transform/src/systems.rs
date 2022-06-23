fn handle_layer_created_events(
    mut layer_created_event_reader: bevy::ecs::event::EventReader<rgis_events::LayerCreatedEvent>,
    layers: bevy::ecs::system::Res<rgis_layers::Layers>,
    rgis_settings: bevy::ecs::system::Res<rgis_settings::RgisSettings>,
    mut task_spawner: rgis_task::TaskSpawner,
) {
    for event in layer_created_event_reader.iter() {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        task_spawner.spawn(crate::tasks::ReprojectGeometryTask {
            geometry: geo::Geometry::GeometryCollection(
                layer.unprojected_feature.to_geometry_collection(),
            ),
            layer_id: event.0,
            source_crs: layer.crs.clone(),
            target_crs: rgis_settings.target_crs.clone(),
        })
    }
}

fn handle_reproject_geometry_task_completion_events(
    mut finished_tasks: bevy::ecs::system::ResMut<rgis_task::FinishedTasks>,
    mut layers: bevy::ecs::system::ResMut<rgis_layers::Layers>,
    mut layer_reprojected_event_writer: bevy::ecs::event::EventWriter<
        rgis_events::LayerReprojectedEvent,
    >,
    rgis_settings: bevy::ecs::system::Res<rgis_settings::RgisSettings>,
) {
    while let Some(outcome) = finished_tasks.take_next::<crate::tasks::ReprojectGeometryTask>() {
        let outcome = match outcome {
            Ok(o) => o,
            Err(e) => {
                bevy::log::error!("Encountered an error reprojecting geometry: {:?}", e);
                continue;
            }
        };

        if outcome.target_crs != rgis_settings.target_crs {
            bevy::log::error!("Encountered a reprojected geometry with a different CRS than the current target CRS");
            continue;
        }

        let layer = match layers.get_mut(outcome.layer_id) {
            Some(l) => l,
            None => continue,
        };

        let feature = match rgis_layers::FeatureCollection::from_geometry(outcome.geometry) {
            Ok(o) => o,
            Err(e) => {
                bevy::log::error!("Encountered an error generating a feature: {:?}", e);
                continue;
            }
        };

        layer.projected_feature = Some(feature);

        layer_reprojected_event_writer.send(rgis_events::LayerReprojectedEvent(outcome.layer_id));
    }
}

fn handle_crs_changed_events(
    mut crs_changed_event_reader: bevy::ecs::event::EventReader<rgis_events::CrsChangedEvent>,
    mut layers: bevy::ecs::system::ResMut<rgis_layers::Layers>,
    rgis_settings: bevy::ecs::system::Res<rgis_settings::RgisSettings>,
    mut task_spawner: rgis_task::TaskSpawner,
) {
    for _ in crs_changed_event_reader.iter() {
        layers.clear_projected();

        for layer in layers.iter() {
            task_spawner.spawn(crate::tasks::ReprojectGeometryTask {
                geometry: geo::Geometry::GeometryCollection(
                    layer.unprojected_feature.to_geometry_collection(),
                ),
                layer_id: layer.id,
                source_crs: layer.crs.clone(),
                target_crs: rgis_settings.target_crs.clone(),
            })
        }
    }
}

pub fn system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new()
        .with_system(handle_layer_created_events)
        .with_system(handle_reproject_geometry_task_completion_events)
        .with_system(handle_crs_changed_events)
}
