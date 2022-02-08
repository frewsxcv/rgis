use bevy::{app::Events, prelude::*};
use geo_bevy::BuildBevyMeshes;
use std::collections;

#[derive(Default)]
struct EntityStore(collections::HashMap<rgis_layers::LayerId, bevy::ecs::entity::Entity>);

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: rgis_layers::ResLayers,
    mut event_reader: EventReader<rgis_layers::LayerLoaded>,
    mut spawned_events: ResMut<Events<rgis_layers::LayerSpawned>>,
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
                &mut meshes,
                &mut commands,
                &mut entity_store,
                layer.id,
            );
        }
        tl.finish();

        spawned_events.send(rgis_layers::LayerSpawned(event.0));
    }
}

pub struct RgisRendererPlugin;

impl Plugin for RgisRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer_spawned)
            .add_system(layer_loaded)
            .add_system(remove_material_event)
            .add_event::<AddMaterialEvent>()
            .add_event::<RemoveMaterialEvent>()
            .insert_resource(EntityStore::default());
    }
}

// System
fn layer_spawned(
    layers: rgis_layers::ResLayers,
    mut camera_offset: ResMut<rgis_camera::CameraOffset>,
    mut camera_scale: ResMut<rgis_camera::CameraScale>,
    mut event_reader: EventReader<rgis_layers::LayerSpawned>,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let layer_center = layer.projected_bounding_rect.rect.center();
        // TODO: this scale math is inprecise. it should take into account
        // .     the height of the geometry. as well as the window size.
        let scale = layer.projected_bounding_rect.rect.width() / 1_000.;
        // TODO: only change the transform if there were no layers previously
        debug!("Moving camera to look at new layer");
        camera_offset.x = layer_center.x as f32;
        camera_offset.y = layer_center.y as f32;
        camera_scale.0 = scale as f32;
    }
}

fn remove_material_event(
    layers: rgis_layers::ResLayers,
    mut event_reader: EventReader<RemoveMaterialEvent>,
    mut entity_store: ResMut<EntityStore>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        let entity = match entity_store.0.remove(&layer.id) {
            Some(h) => h,
            None => continue,
        };
        let mut entity_commands = commands.entity(entity);
        entity_commands.despawn();
    }
}

pub struct RemoveMaterialEvent(pub rgis_layers::LayerId);

pub struct AddMaterialEvent(pub rgis_layers::LayerId);

fn spawn_mesh(
    mesh: Mesh,
    material: Handle<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    entity_store: &mut EntityStore,
    layer_id: rgis_layers::LayerId,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
        ..Default::default()
    };
    let entity_commands = commands.spawn_bundle(mmb);
    entity_store.0.insert(layer_id, entity_commands.id());
}
