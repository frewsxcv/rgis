use bevy::{app::Events, prelude::*};
use geo_bevy::BuildBevyMeshes;

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::ArcLayers>,
    mut event_reader: EventReader<rgis_events::LayerLoadedEvent>,
    mut center_camera_events: ResMut<Events<rgis_events::CenterCameraEvent>>,
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
            layer,
            &mut commands,
            &mut meshes,
        );
        center_camera_events.send(rgis_events::CenterCameraEvent(layer.id));
    }
}

pub struct RgisRendererPlugin;

impl Plugin for RgisRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer_loaded)
            .add_system(toggle_material_event)
            .add_system(handle_layer_color_changed_event);
    }
}

fn spawn_geometry_mesh(
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) {
    let material = materials.add(layer.color.into());

    let tl = time_logger::start(&format!("Triangulating and building {} mesh", layer.name));
    for mesh in layer
        .projected_geometry
        .build_bevy_meshes(geo_bevy::BuildBevyMeshesContext::new())
    {
        spawn_mesh(mesh, material.clone(), meshes, commands, layer.id);
    }
    tl.finish();
}

fn toggle_material_event(
    layers: Res<rgis_layers::ArcLayers>,
    mut event_reader: EventReader<rgis_events::ToggleMaterialEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&rgis_layer_id::LayerId, Entity), With<Handle<ColorMaterial>>>,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        match event {
            rgis_events::ToggleMaterialEvent::Show(layer_id) => {
                let layer = match layers.get(*layer_id) {
                    Some(l) => l,
                    None => continue,
                };

                spawn_geometry_mesh(
                    &mut materials,
                    layer,
                    &mut commands,
                    &mut meshes,
                );
            }
            rgis_events::ToggleMaterialEvent::Hide(layer_id) => {
                for entity in query
                    .iter()
                    .filter_map(|(i, entity)| (i == layer_id).then(|| entity))
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn handle_layer_color_changed_event(
    mut events: EventReader<rgis_events::LayerColorUpdated>,
    layers: Res<rgis_layers::ArcLayers>,
    query: Query<(&rgis_layer_id::LayerId, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in events.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        for handle in query
            .iter()
            .filter_map(|(i, handle)| (*i == event.0).then(|| handle))
        {
            materials.get_mut(handle).unwrap().color = layer.color;
        }
    }
}

fn spawn_mesh(
    mesh: Mesh,
    material: Handle<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
        ..Default::default()
    };
    commands.spawn_bundle(mmb).insert(layer_id);
}
