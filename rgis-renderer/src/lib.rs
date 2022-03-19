use bevy::prelude::*;

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerLoadedEvent>,
    mut center_camera_events: EventWriter<rgis_events::CenterCameraEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.iter() {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        if !layer.visible {
            continue;
        }

        spawn_geometry_mesh(&mut materials, layer, &mut commands, &mut meshes, &asset_server);
        center_camera_events.send(rgis_events::CenterCameraEvent(layer.id));
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer_loaded)
            .add_system(handle_layer_became_hidden_event)
            .add_system(handle_layer_became_visible_event)
            .add_system(handle_layer_color_changed_event)
            .add_system(handle_layer_deleted_events);
    }
}

fn handle_layer_deleted_events(
    mut layer_deleted_event_reader: bevy::app::EventReader<rgis_events::LayerDeleted>,
    mut commands: Commands,
    query: Query<(&rgis_layer_id::LayerId, Entity), With<Handle<ColorMaterial>>>,
) {
    for event in layer_deleted_event_reader.iter() {
        for entity in query
            .iter()
            .filter_map(|(i, entity)| (*i == event.0).then(|| entity))
        {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_geometry_mesh(
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    asset_server: &AssetServer,
) {
    let material = materials.add(layer.color.into());

    let tl = time_logger::start!("Triangulating and building {} mesh", layer.name);
    for mesh in geo_bevy::build_bevy_meshes(
        &layer.projected_geometry,
        geo_bevy::BuildBevyMeshesContext::new(),
    ) {
        match mesh {
            geo_bevy::MeshType::Point(m) => {
                use geo::algorithm::coords_iter::CoordsIter;
                let image_handle = asset_server.load("circle.png");
                for coord in layer.projected_geometry.coords_iter() {
                    let mut transform = Transform::from_xyz(coord.x as f32, coord.y as f32, 0.);
                    transform.scale = (1., 1., 1.,).into();
                    let bundle = SpriteBundle {
                        texture: image_handle.clone(),
                        transform: transform,
                        ..Default::default()
                    };
                    commands.spawn_bundle(bundle).insert(layer.id);
                }
            },
            geo_bevy::MeshType::LineString(m) => {
                spawn_material_mesh_2d_bundle(m, material.clone(), meshes, commands, layer.id);
            },
            geo_bevy::MeshType::Polygon(m) => {
                spawn_material_mesh_2d_bundle(m, material.clone(), meshes, commands, layer.id);
            },
        }
    }
    tl.finish();
}

fn handle_layer_became_hidden_event(
    mut event_reader: EventReader<rgis_events::LayerBecameHiddenEvent>,
    mut commands: Commands,
    query: Query<(&rgis_layer_id::LayerId, Entity), With<Handle<ColorMaterial>>>,
) {
    for event in event_reader.iter() {
        for entity in query
            .iter()
            .filter_map(|(i, entity)| (*i == event.0).then(|| entity))
        {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_layer_became_visible_event(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::LayerBecameVisibleEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.iter() {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        spawn_geometry_mesh(&mut materials, layer, &mut commands, &mut meshes, &asset_server);
    }
}

fn handle_layer_color_changed_event(
    mut events: EventReader<rgis_events::LayerColorUpdated>,
    layers: Res<rgis_layers::Layers>,
    query: Query<(&rgis_layer_id::LayerId, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in events.iter() {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };

        for handle in query
            .iter()
            .filter_map(|(i, handle)| (*i == event.0).then(|| handle))
        {
            match materials.get_mut(handle) {
                Some(color_material) => color_material.color = layer.color,
                None => continue,
            }
        }
    }
}

fn spawn_sprite_bundle(
    mesh: Mesh,
    material: Handle<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
) {
    let bundle = SpriteBundle {
        // mesh: meshes.add(mesh),
        // material: materials.add(Color::PINK.into()),
        // sprite: Sprite::new(Vec2::new(
        //     2.0 * BLOCK_SIZE,
        //     2.0 * BLOCK_SIZE,
        // )),
        // transform: Transform::from_translation(
        //     Vec3::new(0.0, 0.0, 0.0),
        // ),
        ..Default::default()
    };
    commands.spawn_bundle(bundle).insert(layer_id);
}

fn spawn_material_mesh_2d_bundle(
    mesh: Mesh,
    material: Handle<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
) {
    let bundle = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
        ..Default::default()
    };
    commands.spawn_bundle(bundle).insert(layer_id);
}
