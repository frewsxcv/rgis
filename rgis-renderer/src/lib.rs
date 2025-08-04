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
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        systems::configure(app);
    }
}

const SELECTED_COLOR: Color = Color::srgb(255., 192., 203.); // pink

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
    commands
        .spawn((visibility, Transform::default(), layer.id))
        .with_children(|commands| {
            match geometry_mesh {
                geo_bevy::GeometryMesh::Point(points) => {
                    let circle = asset_server.load("circle.png");
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
                        commands
                            .spawn((
                                Sprite {
                                    color: layer.color.stroke,
                                    image: circle.clone(),
                                    ..Default::default()
                                },
                                transform,
                            ))
                            .insert(stroke_entity_type);

                        // Fill
                        let z_index = ZIndex::calculate(layer_index, fill_entity_type);
                        let mut transform = Transform::from_xyz(coord.x, coord.y, z_index.0 as f32);
                        transform.scale *= 0.7; // Fill should be smaller than stroke.
                        commands
                            .spawn((
                                Sprite {
                                    color: if is_selected {
                                        SELECTED_COLOR
                                    } else {
                                        layer.color.fill.unwrap()
                                    },
                                    image: circle.clone(),
                                    ..Default::default()
                                },
                                transform,
                            ))
                            .insert(fill_entity_type);
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
                            match layer.color.fill {
                                Some(color) => color,
                                None => {
                                    bevy::log::error!(
                                        "Expected a fill color for polygon, but none was provided."
                                    );
                                    SELECTED_COLOR
                                }
                            }
                        },
                        layer_index,
                        polygon_mesh.mesh,
                        commands,
                        assets_meshes,
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
                        entity_type,
                    );
                }
            }
        });
}

fn spawn_helper<'a>(
    materials: &'a mut Assets<ColorMaterial>,
    color: bevy::color::Color,
    layer_index: LayerIndex,
    mesh: Mesh,
    commands: &'a mut RelatedSpawnerCommands<'_, ChildOf>,
    assets_meshes: &'a mut Assets<Mesh>,
    entity_type: RenderEntityType,
) -> bevy::ecs::system::EntityCommands<'a> {
    let material = materials.add(color);
    let z_index = ZIndex::calculate(layer_index, entity_type);
    let mut entity_commands = commands.spawn((
        Mesh2d(assets_meshes.add(mesh)),
        Transform::from_xyz(0., 0., z_index.0 as f32),
        MeshMaterial2d(material),
    ));
    entity_commands.insert(entity_type);
    entity_commands
}
