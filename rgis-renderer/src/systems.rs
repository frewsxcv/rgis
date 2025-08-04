use bevy::prelude::*;

use crate::{jobs::MeshBuildingJob, RenderEntityType};

fn layer_loaded(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerReprojectedEvent>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    for layer in event_reader.read().flat_map(|event| layers.get(event.0)) {
        let Some(feature_collection) = layer.projected_feature_collection.as_ref() else {
            continue;
        };

        job_spawner.spawn(MeshBuildingJob {
            layer_id: layer.id,
            geometry: geo::Geometry::GeometryCollection(
                feature_collection.to_geometry_collection(),
            ),
            is_selected: false,
        });
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
        let crate::jobs::MeshBuildingJobOutcome {
            geometry_mesh,
            layer_id,
            is_selected,
        } = match outcome {
            Ok(outcome) => outcome,
            Err(e) => {
                error!("Error processing MeshBuildingJobOutcome: {:?}", e);
                continue;
            }
        };
        let Some(layer_with_index) = layers.get_with_index(layer_id) else {
            continue;
        };

        crate::spawn_geometry_meshes(
            geometry_mesh,
            &mut materials,
            layer_with_index,
            &mut commands,
            &mut assets_meshes,
            &asset_server,
            is_selected,
        );

        meshes_spawned_event_writer.write(layer_id.into());
    }
}

fn handle_layer_z_index_updated_event(
    mut layer_z_index_updated_event_reader: EventReader<rgis_events::LayerZIndexUpdatedEvent>,
    mut query: Query<(&rgis_primitives::LayerId, &mut Transform, &RenderEntityType)>,
    layers: Res<rgis_layers::Layers>,
) {
    for event in layer_z_index_updated_event_reader.read() {
        let Some(layer_with_index) = layers.get_with_index(event.0) else {
            continue;
        };

        for (_, mut transform, render_entity) in query.iter_mut().filter(|(i, _, _)| **i == event.0)
        {
            let z_index = crate::ZIndex::calculate(layer_with_index.1, *render_entity);
            transform.translation.as_mut()[2] = z_index.0 as f32;
        }
    }
}

type LayerEntitiesQuery<'world, 'state, 'a> =
    Query<'world, 'state, (&'a rgis_primitives::LayerId, Entity)>;

fn handle_despawn_meshes_event(
    mut layer_deleted_event_reader: EventReader<rgis_events::DespawnMeshesEvent>,
    mut commands: Commands,
    query: LayerEntitiesQuery,
) {
    for event in layer_deleted_event_reader.read() {
        for (_, entity) in query.iter().filter(|(i, _)| **i == event.0) {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_layer_became_hidden_event(
    mut event_reader: EventReader<rgis_events::LayerBecameHiddenEvent>,
    mut query: Query<(&rgis_primitives::LayerId, &mut Visibility)>,
) {
    for event in event_reader.read() {
        for (_, mut visibility) in query.iter_mut().filter(|(i, _)| **i == event.0) {
            *visibility = Visibility::Hidden;
        }
    }
}

fn handle_layer_became_visible_event(
    mut event_reader: EventReader<rgis_events::LayerBecameVisibleEvent>,
    mut query: Query<(&rgis_primitives::LayerId, &mut Visibility)>,
) {
    for event in event_reader.read() {
        for (_, mut visibility) in query.iter_mut().filter(|(i, _)| **i == event.0) {
            *visibility = Visibility::Visible;
        }
    }
}

fn handle_layer_color_updated_event(
    mut event_reader: EventReader<rgis_events::LayerColorUpdatedEvent>,
    layers: Res<rgis_layers::Layers>,
    color_material_query: Query<(
        &rgis_primitives::LayerId,
        &MeshMaterial2d<ColorMaterial>,
        &RenderEntityType,
    )>,
    mut sprite_query: Query<(&rgis_primitives::LayerId, &mut Sprite, &RenderEntityType)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.read() {
        let (layer_id, is_fill) = match event {
            rgis_events::LayerColorUpdatedEvent::Fill(layer_id) => (layer_id, true),
            rgis_events::LayerColorUpdatedEvent::Stroke(layer_id) => (layer_id, false),
        };
        let Some(layer) = layers.get(*layer_id) else {
            continue;
        };

        if layer.geom_type == geo_geom_type::GeomType::POINT {
            let render_entity_type = if is_fill {
                RenderEntityType::PointFill
            } else {
                RenderEntityType::PointStroke
            };
            for (_, mut sprite, _) in sprite_query.iter_mut().filter(|(i, _, entity_type)| {
                **i == layer.id && **entity_type == render_entity_type
            }) {
                sprite.color = if is_fill {
                    layer.color.fill.unwrap()
                } else {
                    layer.color.stroke
                };
            }
        } else if is_fill {
            for (_, handle, _) in color_material_query.iter().filter(|(i, _, entity_type)| {
                **i == layer.id && **entity_type == RenderEntityType::Polygon
            }) {
                if let Some(color_material) = materials.get_mut(handle) {
                    color_material.color = layer.color.fill.unwrap();
                }
            }
        } else {
            for (_, handle, _) in color_material_query.iter().filter(|(i, _, entity_type)| {
                **i == layer.id && **entity_type == RenderEntityType::LineString
            }) {
                if let Some(color_material) = materials.get_mut(handle) {
                    color_material.color = layer.color.stroke;
                }
            }
        }
    }
}

fn handle_crs_changed_events(
    mut crs_changed_event_reader: EventReader<rgis_events::CrsChangedEvent>,
    query: Query<(&rgis_primitives::LayerId, Entity), With<MeshMaterial2d<ColorMaterial>>>,
    mut commands: Commands,
) {
    for _ in crs_changed_event_reader.read() {
        // FIXME: there's a race condition here where we'll delete newly generated projected geometry
        // meshes if this gets executed after we project the new geometries. We should add a filter
        // in here for the old CRS.
        for (_, entity) in &query {
            commands.entity(entity).despawn();
        }
    }
}

type CameraGlobalTransformQuery<'world, 'state, 'a> =
    Query<'world, 'state, &'a GlobalTransform, (With<Camera>, Changed<GlobalTransform>)>;

fn handle_camera_scale_changed_event(
    query: CameraGlobalTransformQuery,
    mut sprite_bundle_query: Query<&mut Sprite>,
) {
    if let Ok(camera_global_transform) = query.single() {
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
    Or<(With<MeshMaterial2d<ColorMaterial>>, With<Sprite>)>,
>;

fn handle_feature_selected_event_despawn(
    event_reader: EventReader<rgis_events::FeatureSelectedEvent>,
    mut commands: Commands,
    query: SelectedFeatureQuery,
) {
    if !event_reader.is_empty() {
        for (entity, entity_type) in query.iter() {
            match entity_type {
                RenderEntityType::SelectedPolygon
                | RenderEntityType::SelectedLineString
                | RenderEntityType::SelectedPoint => commands.entity(entity).despawn(),
                _ => (),
            }
        }
    }
}

fn handle_feature_selected_event_spawn(
    mut event_reader: EventReader<rgis_events::FeatureSelectedEvent>,
    layers: Res<rgis_layers::Layers>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    for event in event_reader.read() {
        let Some(layer) = layers.get(event.0) else {
            return;
        };
        let Some(feature) = layer.get_projected_feature(event.1) else {
            return;
        };
        let Some(geometry) = feature.geometry.as_ref() else {
            return;
        };
        job_spawner.spawn(MeshBuildingJob {
            layer_id: event.0,
            geometry: geometry.clone(),
            is_selected: true,
        });
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            layer_loaded,
            handle_layer_became_hidden_event,
            handle_layer_became_visible_event,
            handle_layer_color_updated_event,
            handle_layer_z_index_updated_event,
            handle_despawn_meshes_event,
            handle_mesh_building_job_outcome,
            handle_crs_changed_events,
            handle_camera_scale_changed_event,
            handle_feature_selected_event_despawn,
            handle_feature_selected_event_spawn,
        ),
    );
}
