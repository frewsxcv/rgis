#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod jobs;
mod systems;
mod z_index;

use z_index::ZIndex;

#[derive(Clone, Copy, Component, PartialEq, Eq)]
pub enum RenderEntityType {
    Polygon,
    LineString,
    PointFill,
    PointStroke,
    SelectedPolygon,
    SelectedLineString,
    SelectedPoint,
    Raster,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        systems::configure(app);
    }
}

use bevy::render::render_asset::RenderAssetUsages;

const SELECTED_COLOR: Color = Color::srgb(255., 192., 203.); // pink

fn spawn_raster(
    raster: &geo_raster::Raster,
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    layer_index: rgis_layers::LayerIndex,
    images: &mut Assets<Image>,
) {
    let format = match raster.format {
        geo_raster::RasterFormat::R8 => bevy::render::render_resource::TextureFormat::R8Unorm,
        geo_raster::RasterFormat::Rgba8 => bevy::render::render_resource::TextureFormat::Rgba8Unorm,
    };
    let image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: raster.width,
            height: raster.height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        raster.data.clone(),
        format,
        RenderAssetUsages::RENDER_WORLD,
    );
    let image_handle = images.add(image);

    let quad = Mesh::from(Rectangle::new(raster.width as f32, raster.height as f32));

    let material = materials.add(ColorMaterial {
        texture: Some(image_handle),
        ..Default::default()
    });

    let z_index = ZIndex::calculate(layer_index, RenderEntityType::Raster);

    let mut entity_commands = spawn_material_mesh_2d_bundle(
        quad,
        z_index,
        material,
        assets_meshes,
        commands,
        layer.visible,
    );
    entity_commands.insert(layer.id);
    entity_commands.insert(RenderEntityType::Raster);
}

fn spawn_geometry_meshes(
    geometry_mesh: geo_bevy::GeometryMesh,
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    layer_index: rgis_layers::LayerIndex,
    asset_server: &AssetServer,
    is_selected: bool,
) {
    match geometry_mesh {
        geo_bevy::GeometryMesh::Point(points) => {
            for coord in points {
                let (stroke_entity_type, fill_entity_type) = if is_selected {
                    (
                        RenderEntityType::SelectedPoint,
                        RenderEntityType::SelectedPoint,
                    )
                } else {
                    (RenderEntityType::PointStroke, RenderEntityType::PointFill)
                };

                // Stroke
                let z_index = ZIndex::calculate(layer_index, stroke_entity_type);
                let transform = Transform::from_xyz(coord.x, coord.y, z_index.0 as f32);
                let mut entity_commands =
                    spawn_sprite_bundle(asset_server, transform, commands, layer.color.stroke);
                entity_commands.insert(layer.id);
                entity_commands.insert(stroke_entity_type);

                // Fill
                let z_index = ZIndex::calculate(layer_index, fill_entity_type);
                let mut transform = Transform::from_xyz(coord.x, coord.y, z_index.0 as f32);
                transform.scale *= 0.7; // Fill should be smaller than stroke.
                let mut entity_commands = spawn_sprite_bundle(
                    asset_server,
                    transform,
                    commands,
                    if is_selected {
                        SELECTED_COLOR
                    } else {
                        layer.color.fill.unwrap()
                    },
                );
                entity_commands.insert(layer.id);
                entity_commands.insert(fill_entity_type);
            }
        }
        geo_bevy::GeometryMesh::Polygon(polygon_mesh) => {
            let polygon_entity_type = if is_selected {
                RenderEntityType::SelectedPolygon
            } else {
                RenderEntityType::Polygon
            };
            let line_string_entity_type = if is_selected {
                RenderEntityType::SelectedLineString
            } else {
                RenderEntityType::LineString
            };
            // Fill
            spawn_helper(
                materials,
                if is_selected {
                    SELECTED_COLOR
                } else {
                    layer.color.fill.unwrap()
                },
                layer_index,
                polygon_mesh.mesh,
                commands,
                assets_meshes,
                layer,
                polygon_entity_type,
            );
            // Exterior border
            spawn_helper(
                materials,
                layer.color.stroke,
                layer_index,
                polygon_mesh.exterior_mesh,
                commands,
                assets_meshes,
                layer,
                line_string_entity_type,
            );
            // Interior borders
            for mesh in polygon_mesh.interior_meshes {
                spawn_helper(
                    materials,
                    layer.color.stroke,
                    layer_index,
                    mesh,
                    commands,
                    assets_meshes,
                    layer,
                    line_string_entity_type,
                );
            }
        }
        geo_bevy::GeometryMesh::LineString(line_string_mesh) => {
            let entity_type = if is_selected {
                RenderEntityType::SelectedLineString
            } else {
                RenderEntityType::LineString
            };
            spawn_helper(
                materials,
                if is_selected {
                    SELECTED_COLOR
                } else {
                    layer.color.stroke
                },
                layer_index,
                line_string_mesh,
                commands,
                assets_meshes,
                layer,
                entity_type,
            );
        }
    }
}

fn spawn_helper<'a>(
    materials: &'a mut Assets<ColorMaterial>,
    color: bevy::color::Color,
    layer_index: rgis_layers::LayerIndex,
    mesh: Mesh,
    commands: &'a mut Commands<'_, '_>,
    assets_meshes: &'a mut Assets<Mesh>,
    layer: &rgis_layers::Layer,
    entity_type: RenderEntityType,
) -> bevy::ecs::system::EntityCommands<'a> {
    let material = materials.add(color);
    let z_index = ZIndex::calculate(layer_index, entity_type);
    let mut entity_commands = spawn_material_mesh_2d_bundle(
        mesh,
        z_index,
        material,
        assets_meshes,
        commands,
        layer.visible,
    );
    entity_commands.insert(layer.id);
    entity_commands.insert(entity_type);
    entity_commands
}

fn spawn_sprite_bundle<'a>(
    asset_server: &AssetServer,
    transform: Transform,
    commands: &'a mut Commands<'_, '_>,
    color: Color,
) -> bevy::ecs::system::EntityCommands<'a> {
    commands.spawn((
        Sprite {
            color,
            image: asset_server.load("circle.png"),
            ..Default::default()
        },
        transform,
    ))
}

fn spawn_material_mesh_2d_bundle<'a>(
    mesh: Mesh,
    z_index: ZIndex,
    material: Handle<ColorMaterial>,
    assets_meshes: &'a mut Assets<Mesh>,
    commands: &'a mut Commands<'_, '_>,
    is_visible: bool,
) -> bevy::ecs::system::EntityCommands<'a> {
    commands.spawn((
        Mesh2d(assets_meshes.add(mesh)),
        Transform::from_xyz(0., 0., z_index.0 as f32),
        MeshMaterial2d(material),
        if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
    ))
}
