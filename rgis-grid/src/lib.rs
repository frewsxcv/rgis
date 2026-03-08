use bevy::prelude::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid_entity);
        app.add_systems(Update, update_grid);
    }
}

/// Choose a "nice" grid interval based on the camera scale.
///
/// We pick the largest power-of-10 based interval (1, 2, 5 × 10^n) such that
/// grid lines are spaced at least `min_screen_px` pixels apart on screen.
fn nice_interval(camera_scale: f32, _window_size: f32, min_screen_px: f32) -> f32 {
    let min_spacing = camera_scale * min_screen_px;

    let mag = 10f32.powf(min_spacing.log10().floor());
    let normalized = min_spacing / mag;

    if normalized <= 1.0 {
        mag
    } else if normalized <= 2.0 {
        2.0 * mag
    } else if normalized <= 5.0 {
        5.0 * mag
    } else {
        10.0 * mag
    }
}

const MIN_LINE_SPACING_PX: f32 = 80.0;
/// Z-index below all map layers (which start at z=0).
const GRID_Z: f32 = -1.0;

/// Determine grid colors based on the background luminance.
fn grid_colors(clear_color: &ClearColor) -> (Srgba, Srgba) {
    let bg = clear_color.0.to_srgba();
    let luminance = 0.299 * bg.red + 0.587 * bg.green + 0.114 * bg.blue;
    if luminance < 0.5 {
        // Dark mode: subtle light lines
        (
            Srgba::new(1.0, 1.0, 1.0, 0.08),
            Srgba::new(1.0, 1.0, 1.0, 0.15),
        )
    } else {
        // Light mode
        (
            Srgba::new(0.0, 0.0, 0.0, 0.12),
            Srgba::new(0.0, 0.0, 0.0, 0.2),
        )
    }
}

#[derive(Component)]
struct Grid;

fn spawn_grid_entity(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
    ));
    let material = materials.add(Color::WHITE);
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, GRID_Z),
        Grid,
    ));
}

fn update_grid(
    grid_query: Query<(&Mesh2d, &MeshMaterial2d<ColorMaterial>), With<Grid>>,
    camera_query: Query<&Transform, With<Camera>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    clear_color: Res<ClearColor>,
) {
    let Ok(transform) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((mesh_handle, material_handle)) = grid_query.single() else {
        return;
    };

    let camera_scale = transform.scale.x;
    let cam_x = transform.translation.x;
    let cam_y = transform.translation.y;

    let half_w = window.width() * camera_scale * 0.5;
    let half_h = window.height() * camera_scale * 0.5;

    let world_left = cam_x - half_w;
    let world_right = cam_x + half_w;
    let world_bottom = cam_y - half_h;
    let world_top = cam_y + half_h;

    let interval = nice_interval(camera_scale, window.width(), MIN_LINE_SPACING_PX);
    let (grid_color, axis_color) = grid_colors(&clear_color);

    // Line thickness in world units (1 pixel wide)
    let thickness = camera_scale;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let height = world_top - world_bottom;
    let center_y = (world_top + world_bottom) * 0.5;
    let width = world_right - world_left;
    let center_x = (world_right + world_left) * 0.5;

    // vertical lines
    let first_x = (world_left / interval).floor() as i64;
    let last_x = (world_right / interval).ceil() as i64;
    for i in first_x..=last_x {
        let x = i as f32 * interval;
        let color = if i == 0 { axis_color } else { grid_color };
        add_rect(&mut positions, &mut colors, &mut indices, x, center_y, thickness, height, color);
    }

    // horizontal lines
    let first_y = (world_bottom / interval).floor() as i64;
    let last_y = (world_top / interval).ceil() as i64;
    for i in first_y..=last_y {
        let y = i as f32 * interval;
        let color = if i == 0 { axis_color } else { grid_color };
        add_rect(&mut positions, &mut colors, &mut indices, center_x, y, width, thickness, color);
    }

    // Update the mesh in place
    if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(bevy::mesh::Indices::U32(indices));
    }

    // Use a white material; vertex colors provide the actual color
    if let Some(material) = materials.get_mut(&material_handle.0) {
        material.color = Color::WHITE;
    }
}

fn add_rect(
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
    color: Srgba,
) {
    let base = positions.len() as u32;
    let hw = w * 0.5;
    let hh = h * 0.5;
    let c = [color.red, color.green, color.blue, color.alpha];

    positions.push([cx - hw, cy - hh, 0.0]);
    positions.push([cx + hw, cy - hh, 0.0]);
    positions.push([cx + hw, cy + hh, 0.0]);
    positions.push([cx - hw, cy + hh, 0.0]);

    colors.push(c);
    colors.push(c);
    colors.push(c);
    colors.push(c);

    indices.push(base);
    indices.push(base + 1);
    indices.push(base + 2);
    indices.push(base);
    indices.push(base + 2);
    indices.push(base + 3);
}
