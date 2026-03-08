use bevy::prelude::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_grid);
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

const GRID_COLOR: Color = Color::srgba(0.75, 0.75, 0.75, 0.5);
const AXIS_COLOR: Color = Color::srgba(0.6, 0.6, 0.6, 0.7);
const MIN_LINE_SPACING_PX: f32 = 80.0;

fn draw_grid(
    mut gizmos: Gizmos,
    camera_query: Query<&Transform, With<Camera>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Ok(transform) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
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

    // vertical lines
    let first_x = (world_left / interval).floor() as i64;
    let last_x = (world_right / interval).ceil() as i64;
    for i in first_x..=last_x {
        let x = i as f32 * interval;
        let color = if i == 0 { AXIS_COLOR } else { GRID_COLOR };
        gizmos.line_2d(
            Vec2::new(x, world_bottom),
            Vec2::new(x, world_top),
            color,
        );
    }

    // horizontal lines
    let first_y = (world_bottom / interval).floor() as i64;
    let last_y = (world_top / interval).ceil() as i64;
    for i in first_y..=last_y {
        let y = i as f32 * interval;
        let color = if i == 0 { AXIS_COLOR } else { GRID_COLOR };
        gizmos.line_2d(
            Vec2::new(world_left, y),
            Vec2::new(world_right, y),
            color,
        );
    }
}
