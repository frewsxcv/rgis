use bevy::prelude::*;

use crate::{jobs::MeshBuildingJob, RenderEntityType};

fn layer_loaded(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_layer_events::LayerReprojectedEvent>,
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
    mut meshes_spawned_event_writer: EventWriter<rgis_renderer_events::MeshesSpawnedEvent>,
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

// TODO
fn handle_layer_z_index_updated_event(
    mut layer_z_index_updated_event_reader: EventReader<rgis_layer_events::LayerZIndexUpdatedEvent>,
    mut query: Query<(&rgis_primitives::LayerId, &mut Transform, &RenderEntityType)>,
    layers: Res<rgis_layers::Layers>,
) {
    for event in layer_z_index_updated_event_reader.read() {
        let Some(layer_with_index) = layers.get_with_index(event.0) else {
            continue;
        };

        for (layer_id, mut transform, render_entity) in query.iter_mut() {
            if *layer_id == event.0 {
                let z_index = crate::ZIndex::calculate(layer_with_index.1, *render_entity);
                transform.translation.z = z_index.0 as f32;
            }
        }
    }
}

fn handle_layer_point_size_updated_event(
    mut events: EventReader<rgis_layer_events::LayerPointSizeUpdatedEvent>,
    layers: Res<rgis_layers::Layers>,
    mut sprite_query: Query<(
        &mut Sprite,
        &rgis_primitives::LayerId,
        &crate::PointSprite,
    )>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
) {
    let changed_layers: std::collections::HashSet<rgis_primitives::LayerId> =
        events.read().map(|event| event.0).collect();

    if changed_layers.is_empty() {
        return;
    }

    let camera_transform = camera_query.single().unwrap();
    let (camera_scale, _, _) = camera_transform.to_scale_rotation_translation();

    for (mut sprite, layer_id, point_sprite) in sprite_query.iter_mut() {
        if changed_layers.contains(layer_id) {
            if let Some(layer) = layers.get(*layer_id) {
                sprite.custom_size = Some(
                    camera_scale.truncate() * layer.point_size * point_sprite.relative_scale,
                );
            }
        }
    }
}

type LayerEntitiesQuery<'world, 'state, 'a> =
    Query<'world, 'state, (&'a rgis_primitives::LayerId, Entity)>;

fn handle_despawn_meshes_event(
    mut layer_deleted_event_reader: EventReader<rgis_renderer_events::DespawnMeshesEvent>,
    mut commands: Commands,
    query: LayerEntitiesQuery,
) {
    for event in layer_deleted_event_reader.read() {
        for (layer_id, entity) in query.iter() {
            if *layer_id == event.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn handle_layer_became_hidden_event(
    mut event_reader: EventReader<rgis_layer_events::LayerBecameHiddenEvent>,
    mut query: Query<(&rgis_primitives::LayerId, &mut Visibility)>,
) {
    for event in event_reader.read() {
        for (_, mut visibility) in query.iter_mut().filter(|(i, _)| **i == event.0) {
            *visibility = Visibility::Hidden;
        }
    }
}

fn handle_layer_became_visible_event(
    mut event_reader: EventReader<rgis_layer_events::LayerBecameVisibleEvent>,
    mut query: Query<(&rgis_primitives::LayerId, &mut Visibility)>,
) {
    for event in event_reader.read() {
        for (_, mut visibility) in query.iter_mut().filter(|(i, _)| **i == event.0) {
            *visibility = Visibility::Visible;
        }
    }
}

fn handle_layer_color_updated_event(
    mut event_reader: EventReader<rgis_layer_events::LayerColorUpdatedEvent>,
    layers: Res<rgis_layers::Layers>,
    mut point_layer_query: Query<(&rgis_primitives::LayerId, &Children), With<crate::Point>>,
    mut polygon_layer_query: Query<(&rgis_primitives::LayerId, &Children), With<crate::Polygon>>,
    mut line_string_layer_query: Query<
        (&rgis_primitives::LayerId, &Children),
        With<crate::LineString>,
    >,
    mut sprite_fill_query: Query<&mut Sprite, (With<crate::Fill>, Without<crate::Stroke>)>,
    mut sprite_stroke_query: Query<&mut Sprite, (With<crate::Stroke>, Without<crate::Fill>)>,
    mut material_fill_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (With<crate::Fill>, Without<crate::Stroke>),
    >,
    mut material_stroke_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (With<crate::Stroke>, Without<crate::Fill>),
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.read() {
        let (layer_id, is_fill) = match event {
            rgis_layer_events::LayerColorUpdatedEvent::Fill(layer_id) => (layer_id, true),
            rgis_layer_events::LayerColorUpdatedEvent::Stroke(layer_id) => (layer_id, false),
        };
        let Some(layer) = layers.get(*layer_id) else {
            continue;
        };

        // Update the point sprites
        for child in point_layer_query
            .iter_mut()
            .filter(|(i, _children)| **i == layer.id)
            .flat_map(|(_, children)| children.iter())
        {
            if is_fill {
                if let Ok(mut sprite) = sprite_fill_query.get_mut(child) {
                    sprite.color = layer.color.fill.unwrap();
                }
            } else {
                if let Ok(mut sprite) = sprite_stroke_query.get_mut(child) {
                    sprite.color = layer.color.stroke;
                }
            }
        }

        // Update the line string materials
        for child in line_string_layer_query
            .iter_mut()
            .filter(|(i, _children)| **i == layer.id)
            .flat_map(|(_, children)| children.iter())
        {
            if is_fill {
                if let Ok(mut color_material) = material_fill_query.get_mut(child) {
                    materials.get_mut(&mut color_material.0).unwrap().color =
                        layer.color.fill.unwrap();
                }
            } else {
                if let Ok(mut color_material) = material_stroke_query.get_mut(child) {
                    materials.get_mut(&mut color_material.0).unwrap().color = layer.color.stroke;
                }
            }
        }

        // Update the polygon materials
        for child in polygon_layer_query
            .iter_mut()
            .filter(|(i, _children)| **i == layer.id)
            .flat_map(|(_, children)| children.iter())
        {
            if is_fill {
                if let Ok(mut color_material) = material_fill_query.get_mut(child) {
                    materials.get_mut(&mut color_material.0).unwrap().color =
                        layer.color.fill.unwrap();
                }
            } else {
                if let Ok(mut color_material) = material_stroke_query.get_mut(child) {
                    materials.get_mut(&mut color_material.0).unwrap().color = layer.color.stroke;
                }
            }
        }
    }
}

fn handle_crs_changed_events(
    mut crs_changed_event_reader: EventReader<rgis_crs_events::CrsChangedEvent>,
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
    mut sprite_query: Query<(
        &mut Sprite,
        &rgis_primitives::LayerId,
        &crate::PointSprite,
    )>,
    layers: Res<rgis_layers::Layers>,
) {
    if let Ok(camera_global_transform) = query.single() {
        let (scale, _, _) = camera_global_transform.to_scale_rotation_translation();

        for (mut sprite, layer_id, point_sprite) in &mut sprite_query {
            if let Some(layer) = layers.get(*layer_id) {
                sprite.custom_size = Some(
                    scale.truncate() * layer.point_size * point_sprite.relative_scale,
                );
            }
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
    event_reader: EventReader<rgis_map_events::FeatureSelectedEvent>,
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
    mut event_reader: EventReader<rgis_map_events::FeatureSelectedEvent>,
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
            handle_layer_point_size_updated_event,
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
