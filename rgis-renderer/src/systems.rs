use bevy::prelude::*;

use crate::tasks::MeshBuildingTask;

macro_rules! skip_err {
    ($res:expr, $str:literal) => {
        match $res {
            Ok(val) => val,
            Err(error) => {
                bevy::log::error!($str, error);
                continue;
            }
        }
    };
}

macro_rules! skip_none {
    ($res:expr, $str:literal) => {
        match $res {
            Some(val) => val,
            None => {
                bevy::log::error!($str);
                continue;
            }
        }
    };
}

fn layer_loaded(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerReprojectedEvent>,
    mut task_spawner: rgis_task::TaskSpawner,
) {
    for layer in event_reader.iter().flat_map(|event| layers.get(event.0)) {
        let projected_feature = skip_none!(
            layer.projected_feature.as_ref(),
            "Expected a layer to have a projected geometry"
        );

        task_spawner.spawn(crate::tasks::MeshBuildingTask {
            layer_id: layer.id,
            color: layer.color.into(),
            geometry: geo::Geometry::GeometryCollection(projected_feature.to_geometry_collection()),
        })
    }
}

fn handle_mesh_building_task_outcome(
    mut commands: Commands,
    mut assets_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    mut meshes_spawned_event_writer: EventWriter<rgis_events::MeshesSpawnedEvent>,
    mut finished_tasks: ResMut<rgis_task::FinishedTasks>,
) {
    while let Some(outcome) = finished_tasks.take_next::<MeshBuildingTask>() {
        let (meshes, layer_id) = skip_err!(outcome, "Encountered error when spawning mesh: {}");
        let (layer, z_index) =
            skip_none!(layers.get_with_z_index(layer_id), "Could not find layer");

        crate::spawn_geometry_meshes(
            meshes,
            &mut materials,
            layer.id,
            &mut commands,
            &mut assets_meshes,
            z_index,
            layer.visible,
        );

        meshes_spawned_event_writer.send(layer_id.into());
    }
}

fn handle_layer_z_index_updated_event(
    mut layer_z_index_updated_event_reader: bevy::ecs::event::EventReader<
        rgis_events::LayerZIndexUpdatedEvent,
    >,
    mut query: Query<(&rgis_layer_id::LayerId, &mut Transform), With<bevy::sprite::Mesh2dHandle>>,
    layers: Res<rgis_layers::Layers>,
) {
    for event in layer_z_index_updated_event_reader.iter() {
        let (_, z_index) = skip_none!(layers.get_with_z_index(event.0), "Could not find layer");

        for mut transform in query
            .iter_mut()
            .filter_map(|(i, transform)| (*i == event.0).then(|| transform))
        {
            transform.translation[2] = z_index as f32;
        }
    }
}

fn handle_layer_deleted_events(
    mut layer_deleted_event_reader: bevy::ecs::event::EventReader<rgis_events::LayerDeletedEvent>,
    mut commands: Commands,
    query: Query<(&rgis_layer_id::LayerId, Entity), With<Handle<ColorMaterial>>>,
) {
    for event in layer_deleted_event_reader.iter() {
        for (_, entity) in query.iter().filter(|(i, _)| **i == event.0) {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_layer_became_hidden_event(
    mut event_reader: EventReader<rgis_events::LayerBecameHiddenEvent>,
    mut query: Query<(&rgis_layer_id::LayerId, &mut bevy::render::view::Visibility)>,
) {
    for event in event_reader.iter() {
        for (_, mut visibility) in query.iter_mut().filter(|(i, _)| **i == event.0) {
            visibility.is_visible = false;
        }
    }
}

fn handle_layer_became_visible_event(
    mut event_reader: EventReader<rgis_events::LayerBecameVisibleEvent>,
    mut query: Query<(&rgis_layer_id::LayerId, &mut bevy::render::view::Visibility)>,
) {
    for event in event_reader.iter() {
        for (_, mut visibility) in query.iter_mut().filter(|(i, _)| **i == event.0) {
            visibility.is_visible = true;
        }
    }
}

fn handle_layer_color_changed_event(
    mut events: EventReader<rgis_events::LayerColorUpdatedEvent>,
    layers: Res<rgis_layers::Layers>,
    query: Query<(&rgis_layer_id::LayerId, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for layer in events
        .iter()
        .map(|event| event.0)
        .filter_map(|layer_id| layers.get(layer_id))
    {
        for (_, handle) in query.iter().filter(|(i, _)| **i == layer.id) {
            if let Some(color_material) = materials.get_mut(handle) {
                color_material.color = layer.color
            }
        }
    }
}

fn handle_crs_changed_events(
    mut crs_changed_event_reader: bevy::ecs::event::EventReader<rgis_events::CrsChangedEvent>,
    query: Query<(&rgis_layer_id::LayerId, Entity), With<Handle<ColorMaterial>>>,
    mut commands: Commands,
) {
    for _ in crs_changed_event_reader.iter() {
        // FIXME: there's a race condition here where we'll delete newly generated projected geometry
        // meshes if this gets executed after we project the new geometries. We should add a filter
        // in here for the old CRS.
        for (_, entity) in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(layer_loaded)
        .with_system(handle_layer_became_hidden_event)
        .with_system(handle_layer_became_visible_event)
        .with_system(handle_layer_color_changed_event)
        .with_system(handle_layer_z_index_updated_event)
        .with_system(handle_layer_deleted_events)
        .with_system(handle_mesh_building_task_outcome)
        .with_system(handle_crs_changed_events)
}
