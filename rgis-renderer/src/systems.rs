use bevy::prelude::*;

use crate::{jobs::MeshBuildingJob, RenderEntityType};

fn layer_loaded(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerReprojectedEvent>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    for layer in event_reader.iter().flat_map(|event| layers.get(event.0)) {
        let Some(feature_collection) = layer.projected_feature_collection.as_ref() else {
            continue
         };

        job_spawner.spawn(MeshBuildingJob {
            layer_id: layer.id,
            color: layer.color,
            geometry: feature_collection.to_geometry_collection_geometry(),
            is_selected: false,
        })
    }
}

fn handle_mesh_building_job_outcome(
    mut commands: Commands,
    mut assets_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    mut meshes_spawned_event_writer: EventWriter<rgis_events::MeshesSpawnedEvent>,
    mut finished_jobs: bevy_jobs::FinishedJobs,
    asset_server: Res<AssetServer>,
) {
    while let Some(outcome) = finished_jobs.take_next::<MeshBuildingJob>() {
        let Ok(crate::jobs::MeshBuildingJobOutcome {
            prepared_meshes, layer_id, is_selected
        }) = outcome else { continue };
        let Some((layer, layer_index)) = layers.get_with_index(layer_id) else { continue };

        crate::spawn_geometry_meshes(
            prepared_meshes,
            &mut materials,
            layer,
            &mut commands,
            &mut assets_meshes,
            layer_index,
            &asset_server,
            is_selected,
        );

        meshes_spawned_event_writer.send(layer_id.into());
    }
}

fn handle_layer_z_index_updated_event(
    mut layer_z_index_updated_event_reader: bevy::ecs::event::EventReader<
        rgis_events::LayerZIndexUpdatedEvent,
    >,
    mut query: Query<(&rgis_layer_id::LayerId, &mut Transform, &RenderEntityType)>,
    layers: Res<rgis_layers::Layers>,
) {
    for event in layer_z_index_updated_event_reader.iter() {
        let Some((_, layer_index)) = layers.get_with_index(event.0) else { continue };

        for (_, mut transform, render_entity) in query.iter_mut().filter(|(i, _, _)| **i == event.0)
        {
            let z_index = crate::ZIndex::calculate(layer_index, *render_entity);
            transform.translation.as_mut()[2] = z_index.0 as f32;
        }
    }
}

type LayerEntitiesWithColorMaterialsOrImagesQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (&'a rgis_layer_id::LayerId, Entity),
    Or<(With<Handle<ColorMaterial>>, With<Handle<Image>>)>,
>;

fn handle_despawn_meshes_event(
    mut layer_deleted_event_reader: bevy::ecs::event::EventReader<rgis_events::DespawnMeshesEvent>,
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
    color_material_query: Query<(
        &rgis_layer_id::LayerId,
        &Handle<ColorMaterial>,
        &RenderEntityType,
    )>,
    mut sprite_query: Query<(&rgis_layer_id::LayerId, &mut Sprite)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.iter() {
        match event {
            rgis_events::LayerColorUpdatedEvent::Fill(layer_id) => {
                // FIXME: this doesn't handle linestrings
                let Some(layer) = layers.get(*layer_id) else { continue };
                for (_, handle, _) in color_material_query.iter().filter(|(i, _, entity_type)| {
                    **i == layer.id && **entity_type == RenderEntityType::Polygon
                }) {
                    if let Some(color_material) = materials.get_mut(handle) {
                        color_material.color = layer.color
                    }
                }
                for (_, mut sprite) in sprite_query.iter_mut().filter(|(i, _)| **i == layer.id) {
                    sprite.color = layer.color;
                }
            }
            rgis_events::LayerColorUpdatedEvent::Border(layer_id) => {
                let Some(layer) = layers.get(*layer_id) else { continue };
                for (_, handle, _) in color_material_query.iter().filter(|(i, _, entity_type)| {
                    **i == layer.id && **entity_type == RenderEntityType::LineString
                }) {
                    if let Some(color_material) = materials.get_mut(handle) {
                        color_material.color = layer.color
                    }
                }
                for (_, mut sprite) in sprite_query.iter_mut().filter(|(i, _)| **i == layer.id) {
                    sprite.color = layer.color;
                }
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

type SelectedFeatureQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (Entity, &'a RenderEntityType),
    Or<(With<Handle<ColorMaterial>>, With<Handle<Image>>)>,
>;

fn handle_feature_clicked_event(
    mut event_reader: EventReader<rgis_events::FeatureSelectedEvent>,
    layers: Res<rgis_layers::Layers>,
    mut job_spawner: bevy_jobs::JobSpawner,
    mut commands: Commands,
    query: SelectedFeatureQuery,
) {
    for event in event_reader.iter() {
        let Some(layer) = layers.get(event.0) else { return };
        let Some(feature) = layer.get_projected_feature(event.1) else { return };
        let Some(geometry) = feature.geometry() else { return };
        for (entity, entity_type) in query.iter() {
            match entity_type {
                RenderEntityType::SelectedPolygon
                | RenderEntityType::SelectedLineString
                | RenderEntityType::SelectedPoint => commands.entity(entity).despawn(),
                _ => (),
            }
        }
        job_spawner.spawn(MeshBuildingJob {
            layer_id: event.0,
            color: bevy::render::color::Color::PINK,
            geometry: geometry.cloned(),
            is_selected: true,
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
        .with_system(handle_despawn_meshes_event)
        .with_system(handle_mesh_building_job_outcome)
        .with_system(handle_crs_changed_events)
        .with_system(handle_camera_scale_changed_event)
        .with_system(handle_feature_clicked_event)
}
