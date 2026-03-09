use bevy::prelude::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
        app.add_systems(PostUpdate, update_grid);
    }
}

// ── Degree-friendly intervals ────────────────────────────────────────────────

/// Degree-friendly intervals sorted largest → smallest.
/// Whole degrees, then arc-minutes, then arc-seconds.
const DEGREE_INTERVALS: &[f32] = &[
    90.0,
    45.0,
    30.0,
    15.0,
    10.0,
    5.0,
    2.0,
    1.0,
    30.0 / 60.0,
    15.0 / 60.0,
    10.0 / 60.0,
    5.0 / 60.0,
    2.0 / 60.0,
    1.0 / 60.0,
    30.0 / 3600.0,
    15.0 / 3600.0,
    10.0 / 3600.0,
    5.0 / 3600.0,
    2.0 / 3600.0,
    1.0 / 3600.0,
];

/// Pick the smallest degree-friendly interval that keeps at least
/// `min_screen_px` pixels between consecutive grid lines.
fn nice_degree_interval(degrees_per_pixel: f32, min_screen_px: f32) -> f32 {
    let min_degrees = degrees_per_pixel * min_screen_px;
    let mut result = DEGREE_INTERVALS[0];
    for &interval in DEGREE_INTERVALS {
        if interval >= min_degrees {
            result = interval;
        } else {
            break;
        }
    }
    result
}

// ── Generic 1-2-5 interval (fallback for unknown CRS) ───────────────────────

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

// ── Web Mercator helpers (EPSG:3857) ─────────────────────────────────────────

const EARTH_RADIUS: f64 = 6_378_137.0;

fn lon_to_x(lon_deg: f64) -> f32 {
    (lon_deg.to_radians() * EARTH_RADIUS) as f32
}

fn lat_to_y(lat_deg: f64) -> f32 {
    let lat_rad = lat_deg.to_radians();
    (EARTH_RADIUS * (std::f64::consts::FRAC_PI_4 + lat_rad / 2.0).tan().ln()) as f32
}

fn x_to_lon(x: f32) -> f64 {
    (x as f64 / EARTH_RADIUS).to_degrees()
}

fn y_to_lat(y: f32) -> f64 {
    (2.0 * (y as f64 / EARTH_RADIUS).exp().atan() - std::f64::consts::FRAC_PI_2).to_degrees()
}

// ── CRS classification ──────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum CrsKind {
    /// Coordinates are in degrees (e.g. EPSG:4326).
    Geographic,
    /// EPSG:3857 – coordinates in metres, but grid in degrees.
    WebMercator,
    /// Unknown projection – fall back to generic 1-2-5 intervals.
    Other,
}

fn classify_crs(target_crs: &rgis_crs::TargetCrs) -> CrsKind {
    if let Some(code) = target_crs.0.epsg_code {
        if code == 4326 {
            return CrsKind::Geographic;
        }
        if code == 3857 {
            return CrsKind::WebMercator;
        }
        // Look up the proj4 string for other EPSG codes.
        if let Some(def) = crs_definitions::from_code(code) {
            if def.proj4.contains("longlat") {
                return CrsKind::Geographic;
            }
        }
    }
    // Check the raw proj string if available.
    if let Some(ref proj) = target_crs.0.proj_string {
        if proj.contains("longlat") {
            return CrsKind::Geographic;
        }
    }
    CrsKind::Other
}

// ── Label formatting ────────────────────────────────────────────────────────

/// Format a degree value as D°M'S" with N/S or E/W suffix.
fn format_degree(value: f64, is_latitude: bool) -> String {
    let suffix = if is_latitude {
        if value >= 0.0 { "N" } else { "S" }
    } else {
        if value >= 0.0 { "E" } else { "W" }
    };
    let abs = value.abs();
    let deg = abs.floor() as u32;
    let rem = (abs - deg as f64) * 60.0;
    let min = rem.floor() as u32;
    let sec = (rem - min as f64) * 60.0;

    if sec.abs() > 0.01 {
        format!("{deg}\u{00b0}{min}\u{2032}{sec:.0}\u{2033}{suffix}")
    } else if min > 0 {
        format!("{deg}\u{00b0}{min}\u{2032}{suffix}")
    } else {
        format!("{deg}\u{00b0}{suffix}")
    }
}

/// Format a generic projected coordinate value.
fn format_value(value: f32) -> String {
    if value.abs() >= 1_000_000.0 {
        format!("{:.0}", value)
    } else if value.abs() >= 1.0 {
        format!("{:.1}", value)
    } else {
        format!("{:.4}", value)
    }
}

// ── Constants ────────────────────────────────────────────────────────────────

const MIN_LINE_SPACING_PX: f32 = 80.0;
const GRID_Z: f32 = -0.01;
const LABEL_Z: f32 = -0.005;
const LABEL_FONT_SIZE: f32 = 11.0;
const LABEL_MARGIN_PX: f32 = 4.0;

// ── Grid colour ──────────────────────────────────────────────────────────────

fn grid_color(clear_color: &ClearColor) -> Color {
    let bg = clear_color.0.to_srgba();
    let luminance = 0.299 * bg.red + 0.587 * bg.green + 0.114 * bg.blue;
    if luminance < 0.5 {
        Color::srgba(1.0, 1.0, 1.0, 0.08)
    } else {
        Color::srgba(0.0, 0.0, 0.0, 0.12)
    }
}

fn label_color(clear_color: &ClearColor) -> Color {
    let bg = clear_color.0.to_srgba();
    let luminance = 0.299 * bg.red + 0.587 * bg.green + 0.114 * bg.blue;
    if luminance < 0.5 {
        Color::srgba(1.0, 1.0, 1.0, 0.35)
    } else {
        Color::srgba(0.0, 0.0, 0.0, 0.45)
    }
}

// ── Bevy components / state ──────────────────────────────────────────────────

#[derive(Component)]
struct Grid;

#[derive(Component)]
struct GridLabel;

#[derive(Default)]
struct LastCameraState {
    translation: Vec3,
    scale: Vec3,
    window_size: Vec2,
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    mut commands: Commands,
    grid_query: Query<(&Mesh2d, &MeshMaterial2d<ColorMaterial>), With<Grid>>,
    label_query: Query<Entity, With<GridLabel>>,
    camera_query: Query<&Transform, With<Camera>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    clear_color: Res<ClearColor>,
    target_crs: Option<Res<rgis_crs::TargetCrs>>,
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

    let thickness = camera_scale;
    let height = world_top - world_bottom;
    let center_y = (world_top + world_bottom) * 0.5;
    let width = world_right - world_left;
    let center_x = (world_right + world_left) * 0.5;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Despawn old labels
    for entity in label_query.iter() {
        commands.entity(entity).despawn();
    }

    let lbl_color = label_color(&clear_color);
    let margin = LABEL_MARGIN_PX * camera_scale;
    // Position labels at the bottom-left of the viewport with a small inset
    let label_y = world_bottom + margin;
    let label_x = world_left + margin;

    let crs_kind = target_crs
        .as_ref()
        .map(|crs| classify_crs(crs))
        .unwrap_or(CrsKind::Other);

    match crs_kind {
        CrsKind::Geographic => {
            let deg_per_px_x = (world_right - world_left) / window.width();
            let deg_per_px_y = (world_top - world_bottom) / window.height();

            let lon_interval = nice_degree_interval(deg_per_px_x, MIN_LINE_SPACING_PX);
            let lat_interval = nice_degree_interval(deg_per_px_y, MIN_LINE_SPACING_PX);

            // Vertical lines (meridians)
            let first_lon = (world_left / lon_interval).floor() as i64;
            let last_lon = (world_right / lon_interval).ceil() as i64;
            for i in first_lon..=last_lon {
                let x = i as f32 * lon_interval;
                add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
                let text = format_degree(x as f64, false);
                spawn_label(&mut commands, text, x, label_y, camera_scale, lbl_color, false);
            }

            // Horizontal lines (parallels)
            let first_lat = (world_bottom / lat_interval).floor() as i64;
            let last_lat = (world_top / lat_interval).ceil() as i64;
            for i in first_lat..=last_lat {
                let y = i as f32 * lat_interval;
                add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
                let text = format_degree(y as f64, true);
                spawn_label(&mut commands, text, label_x, y, camera_scale, lbl_color, true);
            }
        }

        CrsKind::WebMercator => {
            let lon_left = x_to_lon(world_left);
            let lon_right = x_to_lon(world_right);
            let lat_bottom = y_to_lat(world_bottom);
            let lat_top = y_to_lat(world_top);

            let deg_per_px_lon = (lon_right - lon_left) as f32 / window.width();
            let deg_per_px_lat = (lat_top - lat_bottom) as f32 / window.height();

            let lon_interval = nice_degree_interval(deg_per_px_lon, MIN_LINE_SPACING_PX);
            let lat_interval = nice_degree_interval(deg_per_px_lat, MIN_LINE_SPACING_PX);

            // Vertical lines (meridians)
            let first_lon = (lon_left / lon_interval as f64).floor() as i64;
            let last_lon = (lon_right / lon_interval as f64).ceil() as i64;
            for i in first_lon..=last_lon {
                let lon = i as f64 * lon_interval as f64;
                let x = lon_to_x(lon);
                add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
                let text = format_degree(lon, false);
                spawn_label(&mut commands, text, x, label_y, camera_scale, lbl_color, false);
            }

            // Horizontal lines (parallels)
            let first_lat = (lat_bottom / lat_interval as f64).floor() as i64;
            let last_lat = (lat_top / lat_interval as f64).ceil() as i64;
            for i in first_lat..=last_lat {
                let lat = i as f64 * lat_interval as f64;
                if lat.abs() > 85.051_129 {
                    continue;
                }
                let y = lat_to_y(lat);
                add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
                let text = format_degree(lat, true);
                spawn_label(&mut commands, text, label_x, y, camera_scale, lbl_color, true);
            }
        }

        CrsKind::Other => {
            let interval = nice_interval(camera_scale, MIN_LINE_SPACING_PX);

            let first_x = (world_left / interval).floor() as i64;
            let last_x = (world_right / interval).ceil() as i64;
            for i in first_x..=last_x {
                let x = i as f32 * interval;
                add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
                let text = format_value(x);
                spawn_label(&mut commands, text, x, label_y, camera_scale, lbl_color, false);
            }

            let first_y = (world_bottom / interval).floor() as i64;
            let last_y = (world_top / interval).ceil() as i64;
            for i in first_y..=last_y {
                let y = i as f32 * interval;
                add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
                let text = format_value(y);
                spawn_label(&mut commands, text, label_x, y, camera_scale, lbl_color, true);
            }
        }
    }

    if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(bevy::mesh::Indices::U32(indices));
    }
}

fn spawn_label(
    commands: &mut Commands,
    text: String,
    x: f32,
    y: f32,
    camera_scale: f32,
    color: Color,
    is_latitude: bool,
) {
    let anchor = if is_latitude {
        bevy::sprite::Anchor::CENTER_LEFT
    } else {
        bevy::sprite::Anchor::BOTTOM_CENTER
    };

    // Use a large font size scaled to world units so the text appears at a
    // consistent ~LABEL_FONT_SIZE screen-pixels regardless of zoom.  Bevy's
    // Text2d font_size is in world units; dividing by camera_scale converts
    // the desired screen-pixel size into the current world-unit equivalent.
    let world_font_size = LABEL_FONT_SIZE * camera_scale;

    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: world_font_size,
            ..default()
        },
        TextColor(color),
        anchor,
        Transform::from_xyz(x, y, LABEL_Z),
        bevy::picking::Pickable::IGNORE,
        GridLabel,
    ));
}

fn add_rect(
    positions: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    cx: f32,
    cy: f32,
    w: f32,
    h: f32,
) {
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
