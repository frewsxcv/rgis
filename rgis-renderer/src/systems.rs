use bevy::prelude::*;

use bevy::sprite_render::AlphaMode2d;

use crate::{jobs::MeshBuildingJob, RenderEntityIndex, RenderEntityType};

fn handle_picking_click(
    on: On<Pointer<Click>>,
    layer_query: Query<
        &rgis_primitives::LayerId,
        Or<(With<crate::Point>, With<crate::Polygon>, With<crate::LineString>)>,
    >,
    layer_order: Res<rgis_layers::LayerOrder>,
    layer_data_query: Query<(&rgis_primitives::LayerId, &rgis_layers::LayerData)>,
    current_tool: Res<State<rgis_settings::Tool>>,
    mut bevy_egui_ctx: bevy_egui::EguiContexts,
    mut feature_selected_writer: MessageWriter<rgis_events::FeatureSelectedMessage>,
    mut render_props_writer: MessageWriter<rgis_ui_messages::RenderFeaturePropertiesMessage>,
) {
    if *current_tool.get() != rgis_settings::Tool::Query {
        return;
    }

    if let Ok(ctx) = bevy_egui_ctx.ctx_mut() {
        if ctx.is_pointer_over_area() {
            return;
        }
    }

    // Only handle on parent entities (which have LayerId + geometry marker)
    let Ok(_layer_id) = layer_query.get(on.event_target()) else {
        return;
    };

    // Get the hit position in world space (== projected coordinates)
    let Some(hit_position) = on.event().event.hit.position else {
        return;
    };

    let coord = geo::Coord {
        x: num_t::Num::new(f64::from(hit_position.x)),
        y: num_t::Num::new(f64::from(hit_position.y)),
    };

    // Build an iterator of (LayerId, &LayerData) in top-to-bottom order
    let layers_iter = layer_order.iter_top_to_bottom().filter_map(|entity| {
        layer_data_query.get(entity).ok()
    });
    let layers_vec: Vec<_> = layers_iter.map(|(id, data)| (*id, data)).collect();

    if let Some(result) = rgis_layers::feature_from_click(coord, layers_vec.into_iter()) {
        render_props_writer.write(rgis_ui_messages::RenderFeaturePropertiesMessage {
            layer_id: result.layer_id,
            properties: result.properties,
        });
        feature_selected_writer
            .write(rgis_events::FeatureSelectedMessage(result.layer_id, result.feature.id));
    }
}

fn layer_loaded(
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_order: Res<rgis_layers::LayerOrder>,
    layer_query: Query<(
        &rgis_primitives::LayerId,
        &rgis_layers::LayerVisible,
        &rgis_layers::LayerColor,
        &rgis_layers::LayerPointSize,
        &rgis_layers::LayerData,
    )>,
    mut event_reader: MessageReader<rgis_events::LayerReprojectedMessage>,
    mut job_spawner: bevy_jobs::JobSpawner,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes_spawned_event_writer: MessageWriter<rgis_events::MeshesSpawnedMessage>,
) {
    for event in event_reader.read() {
        let Some(entity) = id_map.get(event.0) else {
            continue;
        };
        let Ok((layer_id, visible, _color, _point_size, data)) = layer_query.get(entity) else {
            continue;
        };
        let layer_index = layer_order
            .index_of(entity)
            .map(rgis_layers::LayerIndex)
            .unwrap_or(rgis_layers::LayerIndex(0));

        match data {
            rgis_layers::LayerData::Raster { raster, projected_grid: Some(grid) } => {
                crate::spawn_raster(
                    raster,
                    grid,
                    *layer_id,
                    visible.0,
                    layer_index,
                    &mut commands,
                    &mut images,
                    &mut meshes,
                    &mut materials,
                );
                meshes_spawned_event_writer.write(event.0.into());
                crate::RENDERED_LAYER_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            rgis_layers::LayerData::Vector {
                projected_feature_collection: Some(feature_collection),
                ..
            } => {
                job_spawner.spawn(MeshBuildingJob {
                    layer_id: *layer_id,
                    geometry: geo::Geometry::GeometryCollection(
                        feature_collection.to_geometry_collection(),
                    ),
                    is_selected: false,
                });
            }
            _ => {}
        }
    }
}

fn handle_mesh_building_job_outcome(
    mut commands: Commands,
    mut assets_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_order: Res<rgis_layers::LayerOrder>,
    layer_query: Query<(
        &rgis_layers::LayerVisible,
        &rgis_layers::LayerColor,
        &rgis_layers::LayerPointSize,
    )>,
    mut meshes_spawned_event_writer: MessageWriter<rgis_events::MeshesSpawnedMessage>,
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
        let Some(entity) = id_map.get(layer_id) else {
            continue;
        };
        let Ok((visible, color, point_size)) = layer_query.get(entity) else {
            continue;
        };
        let layer_index = layer_order
            .index_of(entity)
            .map(rgis_layers::LayerIndex)
            .unwrap_or(rgis_layers::LayerIndex(0));

        crate::spawn_geometry_meshes(
            geometry_mesh,
            &mut materials,
            layer_id,
            visible.0,
            color,
            point_size.0,
            layer_index,
            &mut commands,
            &mut assets_meshes,
            &asset_server,
            is_selected,
        );

        meshes_spawned_event_writer.write(layer_id.into());
        crate::RENDERED_LAYER_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

// TODO
fn handle_layer_z_index_updated_event(
    mut layer_z_index_updated_event_reader: MessageReader<rgis_events::LayerZIndexUpdatedMessage>,
    children_query: Query<&Children>,
    mut transform_query: Query<(&mut Transform, &RenderEntityType)>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_order: Res<rgis_layers::LayerOrder>,
    index: Res<RenderEntityIndex>,
) {
    for event in layer_z_index_updated_event_reader.read() {
        let Some(entity) = id_map.get(event.0) else {
            continue;
        };
        let layer_index = layer_order
            .index_of(entity)
            .map(rgis_layers::LayerIndex)
            .unwrap_or(rgis_layers::LayerIndex(0));

        for &render_entity in index.get(event.0) {
            // Flat entity (e.g. raster): has RenderEntityType directly
            if let Ok((mut transform, render_type)) = transform_query.get_mut(render_entity) {
                let z_index = crate::ZIndex::calculate(layer_index, *render_type);
                transform.translation.z = z_index.0 as f32;
            }
            // Parent-child hierarchy (vector layers): update children
            if let Ok(children) = children_query.get(render_entity) {
                for child in children.iter() {
                    if let Ok((mut transform, render_type)) = transform_query.get_mut(child) {
                        let z_index = crate::ZIndex::calculate(layer_index, *render_type);
                        transform.translation.z = z_index.0 as f32;
                    }
                }
            }
        }
    }
}

fn handle_layer_point_size_updated_event(
    mut events: MessageReader<rgis_events::LayerPointSizeUpdatedMessage>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    point_size_query: Query<&rgis_layers::LayerPointSize>,
    mut sprite_query: Query<(&mut Sprite, &crate::PointSprite)>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    index: Res<RenderEntityIndex>,
) {
    let changed_layers: Vec<rgis_primitives::LayerId> =
        events.read().map(|event| event.0).collect();

    if changed_layers.is_empty() {
        return;
    }

    let camera_transform = camera_query.single().unwrap();
    let (camera_scale, _, _) = camera_transform.to_scale_rotation_translation();

    for layer_id in changed_layers {
        let Some(entity) = id_map.get(layer_id) else {
            continue;
        };
        let Ok(point_size) = point_size_query.get(entity) else {
            continue;
        };
        for &entity in index.get(layer_id) {
            if let Ok((mut sprite, point_sprite)) = sprite_query.get_mut(entity) {
                sprite.custom_size = Some(
                    camera_scale.truncate() * point_size.0 * point_sprite.relative_scale,
                );
            }
        }
    }
}

fn handle_despawn_meshes_event(
    event: On<rgis_events::DespawnMeshesEvent>,
    mut commands: Commands,
    index: Res<RenderEntityIndex>,
) {
    for &entity in index.get(event.0) {
        commands.entity(entity).despawn();
    }
}

fn handle_layer_became_hidden_event(
    event: On<rgis_events::LayerBecameHiddenEvent>,
    mut query: Query<&mut Visibility>,
    index: Res<RenderEntityIndex>,
) {
    for &entity in index.get(event.0) {
        if let Ok(mut visibility) = query.get_mut(entity) {
            *visibility = Visibility::Hidden;
        }
    }
}

fn handle_layer_became_visible_event(
    event: On<rgis_events::LayerBecameVisibleEvent>,
    mut query: Query<&mut Visibility>,
    index: Res<RenderEntityIndex>,
) {
    for &entity in index.get(event.0) {
        if let Ok(mut visibility) = query.get_mut(entity) {
            *visibility = Visibility::Visible;
        }
    }
}

fn handle_point_color_updated_event(
    mut event_reader: MessageReader<rgis_events::LayerColorUpdatedMessage>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    color_query: Query<&rgis_layers::LayerColor>,
    point_layer_query: Query<&Children, With<crate::Point>>,
    mut sprite_fill_query: Query<&mut Sprite, (With<crate::Fill>, Without<crate::Stroke>)>,
    mut sprite_stroke_query: Query<&mut Sprite, (With<crate::Stroke>, Without<crate::Fill>)>,
    index: Res<RenderEntityIndex>,
) {
    for event in event_reader.read() {
        let (layer_id, is_fill) = match event {
            rgis_events::LayerColorUpdatedMessage::Fill(layer_id) => (layer_id, true),
            rgis_events::LayerColorUpdatedMessage::Stroke(layer_id) => (layer_id, false),
        };
        let Some(entity) = id_map.get(*layer_id) else {
            continue;
        };
        let Ok(layer_color) = color_query.get(entity) else {
            continue;
        };

        for &entity in index.get(*layer_id) {
            if let Ok(children) = point_layer_query.get(entity) {
                for child in children.iter() {
                    if is_fill {
                        if let Ok(mut sprite) = sprite_fill_query.get_mut(child) {
                            sprite.color = layer_color.fill.unwrap();
                        }
                    } else {
                        if let Ok(mut sprite) = sprite_stroke_query.get_mut(child) {
                            sprite.color = layer_color.stroke;
                        }
                    }
                }
            }
        }
    }
}

fn handle_line_string_color_updated_event(
    mut event_reader: MessageReader<rgis_events::LayerColorUpdatedMessage>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    color_query: Query<&rgis_layers::LayerColor>,
    line_string_layer_query: Query<&Children, With<crate::LineString>>,
    mut material_fill_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (With<crate::Fill>, Without<crate::Stroke>),
    >,
    mut material_stroke_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (With<crate::Stroke>, Without<crate::Fill>),
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    index: Res<RenderEntityIndex>,
) {
    for event in event_reader.read() {
        let (layer_id, is_fill) = match event {
            rgis_events::LayerColorUpdatedMessage::Fill(layer_id) => (layer_id, true),
            rgis_events::LayerColorUpdatedMessage::Stroke(layer_id) => (layer_id, false),
        };
        let Some(entity) = id_map.get(*layer_id) else {
            continue;
        };
        let Ok(layer_color) = color_query.get(entity) else {
            continue;
        };

        for &entity in index.get(*layer_id) {
            if let Ok(children) = line_string_layer_query.get(entity) {
                for child in children.iter() {
                    if is_fill {
                        if let Ok(mut color_material) = material_fill_query.get_mut(child) {
                            let mat = materials.get_mut(&mut color_material.0).unwrap();
                            let color = layer_color.fill.unwrap();
                            mat.color = color;
                            mat.alpha_mode = alpha_mode_for_color(color);
                        }
                    } else {
                        if let Ok(mut color_material) = material_stroke_query.get_mut(child) {
                            let mat = materials.get_mut(&mut color_material.0).unwrap();
                            mat.color = layer_color.stroke;
                            mat.alpha_mode = alpha_mode_for_color(layer_color.stroke);
                        }
                    }
                }
            }
        }
    }
}

fn handle_polygon_color_updated_event(
    mut event_reader: MessageReader<rgis_events::LayerColorUpdatedMessage>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    color_query: Query<&rgis_layers::LayerColor>,
    polygon_layer_query: Query<&Children, With<crate::Polygon>>,
    mut material_fill_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (With<crate::Fill>, Without<crate::Stroke>),
    >,
    mut material_stroke_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (With<crate::Stroke>, Without<crate::Fill>),
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    index: Res<RenderEntityIndex>,
) {
    for event in event_reader.read() {
        let (layer_id, is_fill) = match event {
            rgis_events::LayerColorUpdatedMessage::Fill(layer_id) => (layer_id, true),
            rgis_events::LayerColorUpdatedMessage::Stroke(layer_id) => (layer_id, false),
        };
        let Some(entity) = id_map.get(*layer_id) else {
            continue;
        };
        let Ok(layer_color) = color_query.get(entity) else {
            continue;
        };

        for &entity in index.get(*layer_id) {
            if let Ok(children) = polygon_layer_query.get(entity) {
                for child in children.iter() {
                    if is_fill {
                        if let Ok(mut color_material) = material_fill_query.get_mut(child) {
                            let mat = materials.get_mut(&mut color_material.0).unwrap();
                            let color = layer_color.fill.unwrap();
                            mat.color = color;
                            mat.alpha_mode = alpha_mode_for_color(color);
                        }
                    } else {
                        if let Ok(mut color_material) = material_stroke_query.get_mut(child) {
                            let mat = materials.get_mut(&mut color_material.0).unwrap();
                            mat.color = layer_color.stroke;
                            mat.alpha_mode = alpha_mode_for_color(layer_color.stroke);
                        }
                    }
                }
            }
        }
    }
}

fn alpha_mode_for_color(color: Color) -> AlphaMode2d {
    if color.alpha() < 1.0 {
        AlphaMode2d::Blend
    } else {
        AlphaMode2d::Opaque
    }
}

fn handle_crs_changed_events(
    _event: On<rgis_events::CrsChangedEvent>,
    query: Query<(&rgis_primitives::LayerId, Entity), With<MeshMaterial2d<ColorMaterial>>>,
    mut commands: Commands,
) {
    // FIXME: there's a race condition here where we'll delete newly generated projected geometry
    // meshes if this gets executed after we project the new geometries. We should add a filter
    // in here for the old CRS.
    for (_, entity) in &query {
        commands.entity(entity).despawn();
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
    id_map: Res<rgis_layers::LayerIdToEntity>,
    point_size_query: Query<&rgis_layers::LayerPointSize>,
) {
    if let Ok(camera_global_transform) = query.single() {
        let (scale, _, _) = camera_global_transform.to_scale_rotation_translation();

        for (mut sprite, layer_id, point_sprite) in &mut sprite_query {
            if let Some(entity) = id_map.get(*layer_id) {
                if let Ok(point_size) = point_size_query.get(entity) {
                    sprite.custom_size = Some(
                        scale.truncate() * point_size.0 * point_sprite.relative_scale,
                    );
                }
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
    event_reader: MessageReader<rgis_events::FeatureSelectedMessage>,
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
    mut event_reader: MessageReader<rgis_events::FeatureSelectedMessage>,
    id_map: Res<rgis_layers::LayerIdToEntity>,
    layer_data_query: Query<&rgis_layers::LayerData>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    for event in event_reader.read() {
        let Some(entity) = id_map.get(event.0) else {
            return;
        };
        let Ok(data) = layer_data_query.get(entity) else {
            return;
        };
        let Some(feature) = data.get_projected_feature(event.0, event.1) else {
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
            handle_point_color_updated_event,
            handle_line_string_color_updated_event,
            handle_polygon_color_updated_event,
            handle_layer_point_size_updated_event,
            handle_layer_z_index_updated_event,
            handle_mesh_building_job_outcome,
            handle_camera_scale_changed_event,
            // Despawn old selection entities before spawning new ones
            (
                handle_feature_selected_event_despawn,
                handle_feature_selected_event_spawn,
            )
                .chain(),
        ),
    );
    app.add_observer(handle_picking_click);
    app.add_observer(handle_layer_became_hidden_event);
    app.add_observer(handle_layer_became_visible_event);
    app.add_observer(handle_despawn_meshes_event);
    app.add_observer(handle_crs_changed_events);
}
