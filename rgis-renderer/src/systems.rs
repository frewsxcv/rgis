use bevy::prelude::*;

use crate::tasks::MeshBuildingTask;

fn layer_loaded(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerReprojectedEvent>,
    mut task_spawner: bevy_jobs::JobSpawner,
) {
    for layer in event_reader.iter().flat_map(|event| layers.get(event.0)) {
        let Some(feature_collection) = layer.projected_feature_collection.as_ref() else {
            continue
         };

        task_spawner.spawn(MeshBuildingTask {
            layer_id: layer.id,
            color: layer.color,
            geometry: feature_collection.to_geometry_collection_geometry(),
        })
    }
}

fn handle_mesh_building_task_outcome(
    mut commands: Commands,
    mut assets_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    mut meshes_spawned_event_writer: EventWriter<rgis_events::MeshesSpawnedEvent>,
    mut finished_tasks: bevy_jobs::FinishedJobs,
    asset_server: Res<AssetServer>,
) {
    while let Some(outcome) = finished_tasks.take_next::<MeshBuildingTask>() {
        let Ok((meshes, layer_id)) = outcome else { continue };
        let Some((layer, z_index)) = layers.get_with_z_index(layer_id) else { continue };

        crate::spawn_geometry_meshes(
            meshes,
            &mut materials,
            layer,
            &mut commands,
            &mut assets_meshes,
            z_index,
            &asset_server,
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
        let Some((_, z_index)) = layers.get_with_z_index(event.0) else { continue };

        for mut transform in query
            .iter_mut()
            .filter_map(|(i, transform)| (*i == event.0).then_some(transform))
        {
            transform.translation.as_mut()[2] = z_index as f32;
        }
    }
}

type LayerEntitiesWithColorMaterialsOrImagesQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (&'a rgis_layer_id::LayerId, Entity),
    Or<(With<Handle<ColorMaterial>>, With<Handle<Image>>)>,
>;

fn handle_layer_deleted_events(
    mut layer_deleted_event_reader: bevy::ecs::event::EventReader<rgis_events::LayerDeletedEvent>,
    mut commands: Commands,
    query: LayerEntitiesWithColorMaterialsOrImagesQuery,
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

fn handle_layer_color_updated_event(
    mut event_reader: bevy::ecs::event::EventReader<rgis_events::LayerColorUpdatedEvent>,
    layers: Res<rgis_layers::Layers>,
    color_material_query: Query<(&rgis_layer_id::LayerId, &Handle<ColorMaterial>)>,
    mut sprite_query: Query<(&rgis_layer_id::LayerId, &mut Sprite)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.iter() {
        let Some(layer) = layers.get(event.0) else { continue };
        for (_, handle) in color_material_query.iter().filter(|(i, _)| **i == layer.id) {
            if let Some(color_material) = materials.get_mut(handle) {
                color_material.color = layer.color
            }
        }
        for (_, mut sprite) in sprite_query.iter_mut().filter(|(i, _)| **i == layer.id) {
            sprite.color = layer.color;
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
        for (_, entity) in &query {
            commands.entity(entity).despawn();
        }
    }
}

type CameraGlobalTransformQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    &'a bevy::transform::components::GlobalTransform,
    (
        bevy::ecs::query::With<bevy::render::camera::Camera>,
        bevy::ecs::query::Changed<bevy::transform::components::GlobalTransform>,
    ),
>;

fn handle_camera_scale_changed_event(
    query: CameraGlobalTransformQuery,
    mut sprite_bundle_query: Query<&mut Sprite>,
) {
    if let Ok(camera_global_transform) = query.get_single() {
        let (scale, _, _) = camera_global_transform.to_scale_rotation_translation();

        for mut sprite in &mut sprite_bundle_query {
            sprite.custom_size = Some(scale.truncate() * 5.);
        }
    }
}

fn handle_feature_clicked_event(
    mut event_reader: EventReader<rgis_events::FeatureSelectedEvent>,
    layers: Res<rgis_layers::Layers>,
    mut task_spawner: bevy_jobs::JobSpawner,
) {
    for event in event_reader.iter() {
        let Some(layer) = layers.get(event.0) else { return };
        let Some(feature) = layer.get_projected_feature(event.1) else { return };
        let Some(geometry) = feature.geometry() else { return };
        task_spawner.spawn(MeshBuildingTask {
            layer_id: event.0,
            color: bevy::render::color::Color::PINK,
            geometry: geometry.cloned(),
        });
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(layer_loaded)
        .with_system(handle_layer_became_hidden_event)
        .with_system(handle_layer_became_visible_event)
        .with_system(handle_layer_color_updated_event)
        .with_system(handle_layer_z_index_updated_event)
        .with_system(handle_layer_deleted_events)
        .with_system(handle_mesh_building_task_outcome)
        .with_system(handle_crs_changed_events)
        .with_system(handle_camera_scale_changed_event)
        .with_system(handle_feature_clicked_event)
}
