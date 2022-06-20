#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;

use rgis_task::Task;

mod tasks;

// System
fn layer_loaded(
    mut commands: Commands,
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerReprojectedEvent>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
) {
    for layer in event_reader.iter().flat_map(|event| layers.get(event.0)) {
        // TODO: do we need this check?
        if !layer.visible {
            continue;
        }

        let projected_geometry = match layer.projected_feature {
            // TODO: remove this clone
            Some(ref projected_feature) => projected_feature.geometry.clone(),
            None => {
                bevy::log::error!("Expected a layer to have a projected geometry");
                continue;
            }
        };

        tasks::MeshBuildingTask {
            layer_id: layer.id,
            geometry: projected_geometry,
        }
        .spawn(&thread_pool, &mut commands);
    }
}

fn handle_mesh_building_task_outcome(
    mut commands: Commands,
    mut assets_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    mut meshes_spawned_event_writer: EventWriter<rgis_events::MeshesSpawnedEvent>,
    mut mesh_building_task_outcome: ResMut<
        bevy::ecs::event::Events<rgis_task::TaskFinishedEvent<tasks::MeshBuildingTask>>,
    >,
) {
    for event in mesh_building_task_outcome.drain() {
        let (meshes, layer_id) = match event.outcome {
            Ok(n) => n,
            Err(e) => {
                bevy::log::error!("Encountered error when spawning mesh: {}", e);
                continue;
            }
        };

        let (layer, z_index) = match layers.get_with_z_index(layer_id) {
            Some(l) => l,
            None => continue,
        };

        spawn_geometry_meshes(
            meshes,
            &mut materials,
            layer,
            &mut commands,
            &mut assets_meshes,
            z_index,
            layer.visible,
        );

        meshes_spawned_event_writer.send(layer_id.into());
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer_loaded)
            .add_system(handle_layer_became_hidden_event)
            .add_system(handle_layer_became_visible_event)
            .add_system(handle_layer_color_changed_event)
            .add_system(handle_layer_z_index_updated_event)
            .add_system(handle_layer_deleted_events)
            .add_system(handle_mesh_building_task_outcome)
            .add_system(handle_crs_changed_events)
            .add_plugin(rgis_task::TaskPlugin::<tasks::MeshBuildingTask>::new());
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
        let (_, z_index) = match layers.get_with_z_index(event.0) {
            Some(l) => l,
            None => continue,
        };

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

fn spawn_geometry_meshes(
    meshes: Vec<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    z_index: usize,
    is_visible: bool,
) {
    let material = materials.add(layer.color.into());

    let tl = time_logger::start!("Triangulating and building {} mesh", layer.name);
    for mesh in meshes {
        spawn_mesh(
            mesh,
            z_index,
            material.clone(),
            assets_meshes,
            commands,
            layer.id,
            is_visible,
        );
    }
    tl.finish();
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

fn spawn_mesh(
    mesh: Mesh,
    z_index: usize,
    material: Handle<ColorMaterial>,
    assets_meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
    is_visible: bool,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(assets_meshes.add(mesh)),
        transform: Transform::from_xyz(0., 0., z_index as f32),
        visibility: bevy::render::view::Visibility { is_visible },
        ..Default::default()
    };
    commands.spawn_bundle(mmb).insert(layer_id);
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
