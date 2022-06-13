#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;
use std::error;

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerLoadedEvent>,
    mut meshes_spawned_event_writer: EventWriter<rgis_events::MeshesSpawnedEvent>,
) {
    for event in event_reader.iter() {
        let (layer, z_index) = match layers.get_with_z_index(event.0) {
            Some(l) => l,
            None => continue,
        };

        if !layer.visible {
            continue;
        }

        match spawn_geometry_meshes(&mut materials, layer, &mut commands, &mut meshes, z_index) {
            Ok(_) => meshes_spawned_event_writer.send(layer.id.into()),
            Err(e) => bevy::log::error!("Encountered error when spawning mesh: {}", e),
        }
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
            .add_system(handle_layer_deleted_events);
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
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    z_index: usize,
) -> Result<(), Box<dyn error::Error>> {
    let material = materials.add(layer.color.into());

    let tl = time_logger::start!("Triangulating and building {} mesh", layer.name);
    for mesh in geo_bevy::build_bevy_meshes(
        &layer.projected_feature.geometry,
        geo_bevy::BuildBevyMeshesContext::new(),
    )? {
        spawn_mesh(mesh, z_index, material.clone(), meshes, commands, layer.id);
    }
    tl.finish();
    Ok(())
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
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
        transform: Transform::from_xyz(0., 0., z_index as f32),
        ..Default::default()
    };
    commands.spawn_bundle(mmb).insert(layer_id);
}
