use bevy::prelude::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_grid);
    }
}

/// Choose a "nice" grid interval based on the camera scale.
///
/// We pick the largest power-of-10 based interval (1, 2, 5 × 10^n) such that
/// grid lines are spaced at least `min_screen_px` pixels apart on screen.
fn nice_interval(camera_scale: f32, window_size: f32, min_screen_px: f32) -> f32 {
    // world units visible across the window dimension
    let world_span = camera_scale * window_size;
    // minimum world-unit spacing so lines are ≥ min_screen_px apart
    let min_spacing = world_span * min_screen_px / window_size;

    // find the order of magnitude
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
/// In dark mode, use subtle lighter lines; in light mode, use subtle darker lines.
fn grid_colors(clear_color: &ClearColor) -> (Color, Color) {
    let bg = clear_color.0.to_srgba();
    let luminance = 0.299 * bg.red + 0.587 * bg.green + 0.114 * bg.blue;
    if luminance < 0.5 {
        // Dark mode: subtle light lines
        let grid = Color::srgba(1.0, 1.0, 1.0, 0.08);
        let axis = Color::srgba(1.0, 1.0, 1.0, 0.15);
        (grid, axis)
    } else {
        // Light mode
        let grid = Color::srgba(0.0, 0.0, 0.0, 0.12);
        let axis = Color::srgba(0.0, 0.0, 0.0, 0.2);
        (grid, axis)
    }
}

#[derive(Component)]
struct GridLine;

fn update_grid(
    mut commands: Commands,
    existing: Query<Entity, With<GridLine>>,
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

    // Despawn previous grid lines
    for entity in &existing {
        commands.entity(entity).despawn();
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

    let interval = nice_interval(camera_scale, window.width(), MIN_LINE_SPACING_PX);
    let (grid_color, axis_color) = grid_colors(&clear_color);

    // Line thickness in world units (1 pixel wide)
    let thickness = camera_scale;

    // vertical lines
    let first_x = (world_left / interval).floor() as i64;
    let last_x = (world_right / interval).ceil() as i64;
    let height = world_top - world_bottom;
    let center_y = (world_top + world_bottom) * 0.5;
    for i in first_x..=last_x {
        let x = i as f32 * interval;
        let color = if i == 0 { axis_color } else { grid_color };
        spawn_line(&mut commands, &mut meshes, &mut materials, x, center_y, thickness, height, color);
    }

    // horizontal lines
    let first_y = (world_bottom / interval).floor() as i64;
    let last_y = (world_top / interval).ceil() as i64;
    let width = world_right - world_left;
    let center_x = (world_right + world_left) * 0.5;
    for i in first_y..=last_y {
        let y = i as f32 * interval;
        let color = if i == 0 { axis_color } else { grid_color };
        spawn_line(&mut commands, &mut meshes, &mut materials, center_x, y, width, thickness, color);
    }
}

fn spawn_line(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
) {
    let mesh = meshes.add(Rectangle::new(width, height));
    let material = materials.add(color);
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(x, y, GRID_Z),
        GridLine,
    ));
}
