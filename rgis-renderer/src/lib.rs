use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

mod jobs;
mod systems;
mod z_index;

use rgis_layers::LayerIndex;
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

const SELECTED_COLOR: Color = Color::srgb(255., 192., 203.); // pink

#[derive(Copy, Clone, Component)]
struct Point;

#[derive(Copy, Clone, Component)]
struct LineString;

#[derive(Copy, Clone, Component)]
struct Polygon;

#[derive(Component)]
pub struct PointSprite {
    pub relative_scale: f32,
}

#[derive(Copy, Clone, Component)]
struct Fill;

#[derive(Copy, Clone, Component)]
struct Stroke;

#[derive(Copy, Clone, Component)]
pub struct RasterSprite;

fn spawn_raster(
    raster: &geo_raster::Raster,
    layer: &rgis_layers::Layer,
    layer_index: LayerIndex,
    commands: &mut Commands,
    images: &mut Assets<Image>,
) {
    let (bevy_format, pixel_bytes) = match raster.format {
        geo_raster::RasterFormat::R8 => {
            // Convert grayscale to RGBA for Bevy
            let mut rgba = Vec::with_capacity(raster.data.len() * 4);
            for &g in &raster.data {
                rgba.push(g);
                rgba.push(g);
                rgba.push(g);
                rgba.push(255);
            }
            (bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, rgba)
        }
        geo_raster::RasterFormat::Rgba8 => {
            (bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, raster.data.clone())
        }
    };

    let image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: raster.width,
            height: raster.height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        pixel_bytes,
        bevy_format,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
    );

    let image_handle = images.add(image);

    let extent = &raster.extent;
    let center_x = (extent.min().x + extent.max().x) / 2.0;
    let center_y = (extent.min().y + extent.max().y) / 2.0;
    let world_width = extent.max().x - extent.min().x;
    let world_height = extent.max().y - extent.min().y;

    let scale_x = world_width as f32 / raster.width as f32;
    let scale_y = world_height as f32 / raster.height as f32;

    let z_index = ZIndex::calculate(layer_index, RenderEntityType::Raster);

    let visibility = if layer.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    commands.spawn((
        Sprite {
            image: image_handle,
            ..Default::default()
        },
        Transform::from_xyz(center_x as f32, center_y as f32, z_index.0 as f32)
            .with_scale(bevy::math::Vec3::new(scale_x, scale_y, 1.0)),
        visibility,
        layer.id,
        RenderEntityType::Raster,
        RasterSprite,
    ));
}

fn spawn_geometry_meshes(
    geometry_mesh: geo_bevy::GeometryMesh,
    materials: &mut Assets<ColorMaterial>,
    rgis_layers::LayerWithIndex(layer, layer_index): rgis_layers::LayerWithIndex,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    asset_server: &AssetServer,
    is_selected: bool,
) {
    let visibility = if layer.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    let mut entity_commands = commands.spawn((visibility, Transform::default(), layer.id));
    match geometry_mesh {
        geo_bevy::GeometryMesh::Point(_) => {
            entity_commands.insert(Point);
        }
        geo_bevy::GeometryMesh::Polygon(_) => {
            entity_commands.insert(Polygon);
        }
        geo_bevy::GeometryMesh::LineString(_) => {
            entity_commands.insert(LineString);
        }
    };
    entity_commands.with_children(|commands| match geometry_mesh {
        geo_bevy::GeometryMesh::Point(points) => {
            spawn_point_geometry(
                commands,
                &points,
                asset_server,
                is_selected,
                layer_index,
                layer,
            );
        }
        geo_bevy::GeometryMesh::Polygon(polygon_mesh) => {
            spawn_polygon_geometry(
                commands,
                polygon_mesh,
                materials,
                assets_meshes,
                is_selected,
                layer_index,
                layer,
            );
        }
        geo_bevy::GeometryMesh::LineString(line_string_mesh) => {
            spawn_linestring_geometry(
                commands,
                line_string_mesh,
                materials,
                assets_meshes,
                is_selected,
                layer_index,
                layer,
            );
        }
    });
}

fn spawn_linestring_geometry(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    line_string_mesh: Mesh,
    materials: &mut Assets<ColorMaterial>,
    assets_meshes: &mut Assets<Mesh>,
    is_selected: bool,
    layer_index: LayerIndex,
    layer: &rgis_layers::Layer,
) {
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
        entity_type,
        false,
    );
}

fn spawn_polygon_geometry(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    polygon_mesh: geo_bevy::PolygonMesh,
    materials: &mut Assets<ColorMaterial>,
    assets_meshes: &mut Assets<Mesh>,
    is_selected: bool,
    layer_index: LayerIndex,
    layer: &rgis_layers::Layer,
) {
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
            match layer.color.fill {
                Some(color) => color,
                None => {
                    bevy::log::error!("Expected a fill color for polygon, but none was provided.");
                    SELECTED_COLOR
                }
            }
        },
        layer_index,
        polygon_mesh.mesh,
        commands,
        assets_meshes,
        polygon_entity_type,
        true,
    );
    // Exterior border
    spawn_helper(
        materials,
        layer.color.stroke,
        layer_index,
        polygon_mesh.exterior_mesh,
        commands,
        assets_meshes,
        line_string_entity_type,
        false,
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
            line_string_entity_type,
            false,
        );
    }
}

fn spawn_point_geometry(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    points: &[geo_bevy::SpritePosition],
    asset_server: &AssetServer,
    is_selected: bool,
    layer_index: LayerIndex,
    layer: &rgis_layers::Layer,
) {
    let circle = asset_server.load("circle.png");
    let (stroke_entity_type, fill_entity_type) = if is_selected {
        (
            RenderEntityType::SelectedPoint,
            RenderEntityType::SelectedPoint,
        )
    } else {
        (RenderEntityType::PointStroke, RenderEntityType::PointFill)
    };
    // Stroke
    spawn_point_sprites(
        commands,
        points,
        layer_index,
        stroke_entity_type,
        layer.color.stroke,
        circle.clone(),
        1.0,
        false,
        layer,
    );

    // Fill
    spawn_point_sprites(
        commands,
        points,
        layer_index,
        fill_entity_type,
        if is_selected {
            SELECTED_COLOR
        } else {
            layer.color.fill.unwrap()
        },
        circle.clone(),
        0.7, // Fill should be smaller than stroke.
        true,
        layer,
    );
}

fn spawn_helper<'a>(
    materials: &'a mut Assets<ColorMaterial>,
    color: bevy::color::Color,
    layer_index: LayerIndex,
    mesh: Mesh,
    commands: &'a mut RelatedSpawnerCommands<ChildOf>,
    assets_meshes: &'a mut Assets<Mesh>,
    entity_type: RenderEntityType,
    is_fill: bool,
) -> bevy::ecs::system::EntityCommands<'a> {
    let material = materials.add(color);
    let z_index = ZIndex::calculate(layer_index, entity_type);
    let mut entity_commands = commands.spawn((
        Mesh2d(assets_meshes.add(mesh)),
        Transform::from_xyz(0., 0., z_index.0 as f32),
        MeshMaterial2d(material),
        entity_type,
    ));
    if is_fill {
        entity_commands.insert(Fill);
    } else {
        entity_commands.insert(Stroke);
    }
    entity_commands
}

fn spawn_point_sprites(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    points: &[geo_bevy::SpritePosition],
    layer_index: LayerIndex,
    entity_type: RenderEntityType,
    color: Color,
    circle: Handle<Image>,
    scale: f32,
    is_fill: bool,
    layer: &rgis_layers::Layer,
) {
    for coord in points {
        let z_index = ZIndex::calculate(layer_index, entity_type);
        let transform = Transform::from_xyz(coord.x, coord.y, z_index.0 as f32);
        let mut entity_commands = commands.spawn((
            Sprite {
                color,
                image: circle.clone(),
                ..Default::default()
            },
            entity_type,
            transform,
            layer.id,
            PointSprite {
                relative_scale: scale,
            },
        ));
        if is_fill {
            entity_commands.insert(Fill);
        } else {
            entity_commands.insert(Stroke);
        }
    }
}
