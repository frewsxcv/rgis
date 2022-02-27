use bevy::{app::Events, prelude::*};
use geo_bevy::BuildBevyMeshes;
use std::collections;

#[derive(Default)]
struct EntityStore(collections::HashMap<rgis_layer_id::LayerId, Vec<bevy::ecs::entity::Entity>>);

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: rgis_layers::ResLayers,
    mut event_reader: EventReader<rgis_events::LayerLoadedEvent>,
    mut center_camera_events: ResMut<Events<rgis_events::CenterCameraEvent>>,
    mut entity_store: ResMut<EntityStore>,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        if !layer.visible {
            continue;
        }

        spawn_geometry_mesh(
            &mut materials,
            &layer,
            &mut commands,
            &mut meshes,
            &mut entity_store,
        );
        center_camera_events.send(rgis_events::CenterCameraEvent(layer.id));
    }
}

pub struct RgisRendererPlugin;

impl Plugin for RgisRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer_loaded)
            .add_system(toggle_material_event)
            .insert_resource(EntityStore::default());
    }
}

fn spawn_geometry_mesh(
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    entity_store: &mut EntityStore,
) {
    let material = materials.add(layer.color.into());

    let tl = time_logger::start(&format!("Triangulating and building {} mesh", layer.name));
    for mesh in layer
        .projected_geometry
        .geometry
        .build_bevy_meshes(geo_bevy::BuildBevyMeshesContext::new())
    {
        spawn_mesh(
            mesh,
            material.clone(),
            meshes,
            commands,
            entity_store,
            layer.id,
        );
    }
    tl.finish();
}

fn toggle_material_event(
    layers: rgis_layers::ResLayers,
    mut event_reader: EventReader<rgis_events::ToggleMaterialEvent>,
    mut entity_store: ResMut<EntityStore>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.iter() {
        match event {
            rgis_events::ToggleMaterialEvent::Show(layer_id) => {
                let layers = layers.read().unwrap();
                let layer = match layers.get(*layer_id) {
                    Some(l) => l,
                    None => continue,
                };

                spawn_geometry_mesh(
                    &mut materials,
                    layer,
                    &mut commands,
                    &mut meshes,
                    &mut entity_store,
                );
            }
            rgis_events::ToggleMaterialEvent::Hide(layer_id) => {
                let layers = layers.read().unwrap();
                let layer = match layers.get(*layer_id) {
                    Some(l) => l,
                    None => continue,
                };

                let entities = match entity_store.0.remove(&layer.id) {
                    Some(h) => h,
                    None => continue,
                };
                for entity in entities {
                    let mut entity_commands = commands.entity(entity);
                    entity_commands.despawn();
                }
            }
        }
    }
}

fn spawn_mesh(
    mesh: Mesh,
    material: Handle<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    entity_store: &mut EntityStore,
    layer_id: rgis_layer_id::LayerId,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
        ..Default::default()
    };
    let entity_commands = commands.spawn_bundle(mmb);
    entity_store
        .0
        .entry(layer_id)
        .or_default()
        .push(entity_commands.id());
}
