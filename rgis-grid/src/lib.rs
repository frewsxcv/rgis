use bevy::prelude::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
        app.add_systems(PostUpdate, update_grid);
    }
}

fn nice_interval(camera_scale: f32, min_screen_px: f32) -> f32 {
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
const GRID_Z: f32 = -1.0;

fn grid_color(clear_color: &ClearColor) -> Color {
    let bg = clear_color.0.to_srgba();
    let luminance = 0.299 * bg.red + 0.587 * bg.green + 0.114 * bg.blue;
    if luminance < 0.5 {
        Color::srgba(1.0, 1.0, 1.0, 0.08)
    } else {
        Color::srgba(0.0, 0.0, 0.0, 0.12)
    }
}

#[derive(Component)]
struct Grid;

#[derive(Default)]
struct LastCameraState {
    translation: Vec3,
    scale: Vec3,
    window_size: Vec2,
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Start with an empty mesh; update_grid will populate it on the first frame.
    let mesh = meshes.add(Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
    ));
    let material = materials.add(Color::srgba(0.0, 0.0, 0.0, 0.12));
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, GRID_Z),
        bevy::picking::Pickable::IGNORE,
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
    mut last_state: Local<LastCameraState>,
) {
    let Ok(transform) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((mesh_handle, mat_handle)) = grid_query.single() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());
    if transform.translation == last_state.translation
        && transform.scale == last_state.scale
        && window_size == last_state.window_size
    {
        return;
    }

    last_state.translation = transform.translation;
    last_state.scale = transform.scale;
    last_state.window_size = window_size;

    // Update material color for current theme
    if let Some(mat) = materials.get_mut(&mat_handle.0) {
        mat.color = grid_color(&clear_color);
    }

    let camera_scale = transform.scale.x;
    let cam_x = transform.translation.x;
    let cam_y = transform.translation.y;

    let half_w = window.width() * camera_scale * 0.5;
    let half_h = window.height() * camera_scale * 0.5;

    let world_left = cam_x - half_w;
    let world_right = cam_x + half_w;
    let world_bottom = cam_y - half_h;
    let world_top = cam_y + half_h;

    let interval = nice_interval(camera_scale, MIN_LINE_SPACING_PX);
    let thickness = camera_scale;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let height = world_top - world_bottom;
    let center_y = (world_top + world_bottom) * 0.5;
    let width = world_right - world_left;
    let center_x = (world_right + world_left) * 0.5;

    let first_x = (world_left / interval).floor() as i64;
    let last_x = (world_right / interval).ceil() as i64;
    for i in first_x..=last_x {
        let x = i as f32 * interval;
        add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
    }

    let first_y = (world_bottom / interval).floor() as i64;
    let last_y = (world_top / interval).ceil() as i64;
    for i in first_y..=last_y {
        let y = i as f32 * interval;
        add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
    }

    // Mutate the existing mesh asset in place
    if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(bevy::mesh::Indices::U32(indices));
    }
}

fn add_rect(positions: &mut Vec<[f32; 3]>, indices: &mut Vec<u32>, cx: f32, cy: f32, w: f32, h: f32) {
    let base = positions.len() as u32;
    let hw = w * 0.5;
    let hh = h * 0.5;
    positions.push([cx - hw, cy - hh, 0.0]);
    positions.push([cx + hw, cy - hh, 0.0]);
    positions.push([cx + hw, cy + hh, 0.0]);
    positions.push([cx - hw, cy + hh, 0.0]);
    indices.push(base);
    indices.push(base + 1);
    indices.push(base + 2);
    indices.push(base);
    indices.push(base + 2);
    indices.push(base + 3);
}
