use bevy::prelude::*;

pub struct Plugin;

#[derive(Resource)]
struct GridFont(Handle<Font>);

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_grid, load_grid_font));
        app.add_systems(PostUpdate, update_grid);
        app.add_systems(Update, update_grid_labels);
    }
}

fn load_grid_font(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font_data = include_bytes!("../../rgis/assets/fonts/RobotoMono-VariableFont_wght.ttf");
    let font = fonts.add(Font::try_from_bytes(font_data.to_vec()).expect("Failed to load embedded font"));
    commands.insert_resource(GridFont(font));
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

/// Format a degree value with N/S or E/W suffix.
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

    if deg == 0 && min == 0 && sec.abs() > 0.01 {
        format!("{sec:.0}\u{2033} {suffix}")
    } else if sec.abs() > 0.01 {
        format!("{deg}\u{00b0}{min}\u{2032}{sec:.0}\u{2033} {suffix}")
    } else if min > 0 {
        format!("{deg}\u{00b0}{min}\u{2032} {suffix}")
    } else {
        format!("{deg}\u{00b0} {suffix}")
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
const LABEL_FONT_SIZE: f32 = 11.0;
const LABEL_MARGIN_PX: f32 = 8.0;

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

// ── Viewport info shared between grid mesh and label systems ─────────────────

struct ViewportInfo {
    camera_scale: f32,
    win_w: f32,
    win_h: f32,
    world_left: f32,
    world_right: f32,
    world_bottom: f32,
    world_top: f32,
}

fn get_viewport_info(
    camera_query: &Query<&Transform, With<Camera>>,
    windows: &Query<&Window, With<bevy::window::PrimaryWindow>>,
) -> Option<ViewportInfo> {
    let Ok(transform) = camera_query.single() else {
        return None;
    };
    let Ok(window) = windows.single() else {
        return None;
    };
    let camera_scale = transform.scale.x;
    let win_w = window.width();
    let win_h = window.height();
    let half_w = win_w * camera_scale * 0.5;
    let half_h = win_h * camera_scale * 0.5;
    let cam_x = transform.translation.x;
    let cam_y = transform.translation.y;
    Some(ViewportInfo {
        camera_scale,
        win_w,
        win_h,
        world_left: cam_x - half_w,
        world_right: cam_x + half_w,
        world_bottom: cam_y - half_h,
        world_top: cam_y + half_h,
    })
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
    grid_query: Query<(&Mesh2d, &MeshMaterial2d<ColorMaterial>), With<Grid>>,
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

    let Some(vp) = get_viewport_info(&camera_query, &windows) else {
        return;
    };

    let thickness = vp.camera_scale;
    let height = vp.world_top - vp.world_bottom;
    let center_y = (vp.world_top + vp.world_bottom) * 0.5;
    let width = vp.world_right - vp.world_left;
    let center_x = (vp.world_right + vp.world_left) * 0.5;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let crs_kind = target_crs
        .as_ref()
        .map(|crs| classify_crs(crs))
        .unwrap_or(CrsKind::Other);

    match crs_kind {
        CrsKind::Geographic => {
            let deg_per_px_x = (vp.world_right - vp.world_left) / vp.win_w;
            let deg_per_px_y = (vp.world_top - vp.world_bottom) / vp.win_h;

            let lon_interval = nice_degree_interval(deg_per_px_x, MIN_LINE_SPACING_PX);
            let lat_interval = nice_degree_interval(deg_per_px_y, MIN_LINE_SPACING_PX);

            let first_lon = (vp.world_left / lon_interval).floor() as i64;
            let last_lon = (vp.world_right / lon_interval).ceil() as i64;
            for i in first_lon..=last_lon {
                let x = i as f32 * lon_interval;
                add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
            }

            let first_lat = (vp.world_bottom / lat_interval).floor() as i64;
            let last_lat = (vp.world_top / lat_interval).ceil() as i64;
            for i in first_lat..=last_lat {
                let y = i as f32 * lat_interval;
                add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
            }
        }

        CrsKind::WebMercator => {
            let lon_left = x_to_lon(vp.world_left);
            let lon_right = x_to_lon(vp.world_right);
            let lat_bottom = y_to_lat(vp.world_bottom);
            let lat_top = y_to_lat(vp.world_top);

            let deg_per_px_lon = (lon_right - lon_left) as f32 / vp.win_w;
            let deg_per_px_lat = (lat_top - lat_bottom) as f32 / vp.win_h;

            let lon_interval = nice_degree_interval(deg_per_px_lon, MIN_LINE_SPACING_PX);
            let lat_interval = nice_degree_interval(deg_per_px_lat, MIN_LINE_SPACING_PX);

            let first_lon = (lon_left / lon_interval as f64).floor() as i64;
            let last_lon = (lon_right / lon_interval as f64).ceil() as i64;
            for i in first_lon..=last_lon {
                let lon = i as f64 * lon_interval as f64;
                let x = lon_to_x(lon);
                add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
            }

            let first_lat = (lat_bottom / lat_interval as f64).floor() as i64;
            let last_lat = (lat_top / lat_interval as f64).ceil() as i64;
            for i in first_lat..=last_lat {
                let lat = i as f64 * lat_interval as f64;
                if lat.abs() > 85.051_129 {
                    continue;
                }
                let y = lat_to_y(lat);
                add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
            }
        }

        CrsKind::Other => {
            let interval = nice_interval(vp.camera_scale, MIN_LINE_SPACING_PX);

            let first_x = (vp.world_left / interval).floor() as i64;
            let last_x = (vp.world_right / interval).ceil() as i64;
            for i in first_x..=last_x {
                let x = i as f32 * interval;
                add_rect(&mut positions, &mut indices, x, center_y, thickness, height);
            }

            let first_y = (vp.world_bottom / interval).floor() as i64;
            let last_y = (vp.world_top / interval).ceil() as i64;
            for i in first_y..=last_y {
                let y = i as f32 * interval;
                add_rect(&mut positions, &mut indices, center_x, y, width, thickness);
            }
        }
    }

    if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(bevy::mesh::Indices::U32(indices));
    }
}

// ── Bevy UI label rendering ─────────────────────────────────────────────────

/// Collected label: screen position + text.
struct LabelSpec {
    screen_x: f32,
    screen_y: f32,
    text: String,
    is_y_axis: bool,
}

fn update_grid_labels(
    mut commands: Commands,
    label_query: Query<Entity, With<GridLabel>>,
    camera_query: Query<&Transform, With<Camera>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    clear_color: Res<ClearColor>,
    target_crs: Option<Res<rgis_crs::TargetCrs>>,
    grid_font: Option<Res<GridFont>>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    bottom_panel_height: Res<rgis_units::BottomPanelHeight>,
    mut last_state: Local<LastCameraState>,
) {
    // Wait for the font to be available before spawning labels.
    let Some(ref font_res) = grid_font else {
        return;
    };

    let Ok(transform) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());
    if transform.translation == last_state.translation
        && transform.scale == last_state.scale
        && window_size == last_state.window_size
        && !label_query.is_empty()
    {
        return;
    }

    last_state.translation = transform.translation;
    last_state.scale = transform.scale;
    last_state.window_size = window_size;

    // Despawn all previous labels.
    for entity in label_query.iter() {
        commands.entity(entity).despawn();
    }

    let Some(vp) = get_viewport_info(&camera_query, &windows) else {
        return;
    };

    let color = label_color(&clear_color);

    // Screen positions for label rows.
    let label_screen_y = vp.win_h - bottom_panel_height.0 - LABEL_MARGIN_PX - 20.0;
    let label_screen_x = side_panel_width.0 + LABEL_MARGIN_PX + 4.0;
    let cam_x = (vp.world_left + vp.world_right) * 0.5;
    let cam_y = (vp.world_bottom + vp.world_top) * 0.5;

    // World → screen coordinate helpers.
    let world_to_screen_x = |wx: f32| -> f32 {
        (wx - cam_x) / vp.camera_scale + vp.win_w / 2.0
    };
    let world_to_screen_y = |wy: f32| -> f32 {
        vp.win_h / 2.0 - (wy - cam_y) / vp.camera_scale
    };

    let crs_kind = target_crs
        .as_ref()
        .map(|crs| classify_crs(crs))
        .unwrap_or(CrsKind::Other);

    let mut labels: Vec<LabelSpec> = Vec::new();

    match crs_kind {
        CrsKind::Geographic => {
            let deg_per_px_x = (vp.world_right - vp.world_left) / vp.win_w;
            let deg_per_px_y = (vp.world_top - vp.world_bottom) / vp.win_h;
            let lon_interval = nice_degree_interval(deg_per_px_x, MIN_LINE_SPACING_PX);
            let lat_interval = nice_degree_interval(deg_per_px_y, MIN_LINE_SPACING_PX);

            let first_lon = (vp.world_left / lon_interval).floor() as i64;
            let last_lon = (vp.world_right / lon_interval).ceil() as i64;
            for i in first_lon..=last_lon {
                let x = i as f32 * lon_interval;
                labels.push(LabelSpec {
                    screen_x: world_to_screen_x(x),
                    screen_y: label_screen_y,
                    text: format_degree(x as f64, false),
                    is_y_axis: false,
                });
            }

            let first_lat = (vp.world_bottom / lat_interval).floor() as i64;
            let last_lat = (vp.world_top / lat_interval).ceil() as i64;
            for i in first_lat..=last_lat {
                let y = i as f32 * lat_interval;
                labels.push(LabelSpec {
                    screen_x: label_screen_x,
                    screen_y: world_to_screen_y(y),
                    text: format_degree(y as f64, true),
                    is_y_axis: true,
                });
            }
        }

        CrsKind::WebMercator => {
            let lon_left = x_to_lon(vp.world_left);
            let lon_right = x_to_lon(vp.world_right);
            let lat_bottom = y_to_lat(vp.world_bottom);
            let lat_top = y_to_lat(vp.world_top);

            let deg_per_px_lon = (lon_right - lon_left) as f32 / vp.win_w;
            let deg_per_px_lat = (lat_top - lat_bottom) as f32 / vp.win_h;

            let lon_interval = nice_degree_interval(deg_per_px_lon, MIN_LINE_SPACING_PX);
            let lat_interval = nice_degree_interval(deg_per_px_lat, MIN_LINE_SPACING_PX);

            let first_lon = (lon_left / lon_interval as f64).floor() as i64;
            let last_lon = (lon_right / lon_interval as f64).ceil() as i64;
            for i in first_lon..=last_lon {
                let lon = i as f64 * lon_interval as f64;
                let x = lon_to_x(lon);
                labels.push(LabelSpec {
                    screen_x: world_to_screen_x(x),
                    screen_y: label_screen_y,
                    text: format_degree(lon, false),
                    is_y_axis: false,
                });
            }

            let first_lat = (lat_bottom / lat_interval as f64).floor() as i64;
            let last_lat = (lat_top / lat_interval as f64).ceil() as i64;
            for i in first_lat..=last_lat {
                let lat = i as f64 * lat_interval as f64;
                if lat.abs() > 85.051_129 {
                    continue;
                }
                let y = lat_to_y(lat);
                labels.push(LabelSpec {
                    screen_x: label_screen_x,
                    screen_y: world_to_screen_y(y),
                    text: format_degree(lat, true),
                    is_y_axis: true,
                });
            }
        }

        CrsKind::Other => {
            let interval = nice_interval(vp.camera_scale, MIN_LINE_SPACING_PX);

            let first_x = (vp.world_left / interval).floor() as i64;
            let last_x = (vp.world_right / interval).ceil() as i64;
            for i in first_x..=last_x {
                let x = i as f32 * interval;
                labels.push(LabelSpec {
                    screen_x: world_to_screen_x(x),
                    screen_y: label_screen_y,
                    text: format_value(x),
                    is_y_axis: false,
                });
            }

            let first_y = (vp.world_bottom / interval).floor() as i64;
            let last_y = (vp.world_top / interval).ceil() as i64;
            for i in first_y..=last_y {
                let y = i as f32 * interval;
                labels.push(LabelSpec {
                    screen_x: label_screen_x,
                    screen_y: world_to_screen_y(y),
                    text: format_value(y),
                    is_y_axis: true,
                });
            }
        }
    }

    // Spawn Bevy UI Text nodes positioned absolutely on screen.
    for label in labels {
        // Skip labels outside visible area.
        if label.screen_x < 0.0 || label.screen_x > vp.win_w
            || label.screen_y < 0.0 || label.screen_y > vp.win_h
        {
            continue;
        }

        let justify = if label.is_y_axis {
            Justify::Left
        } else {
            Justify::Center
        };

        commands.spawn((
            Text::new(label.text),
            TextFont {
                font: font_res.0.clone(),
                font_size: LABEL_FONT_SIZE,
                ..default()
            },
            TextColor(color),
            TextLayout::new_with_justify(justify),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(label.screen_x),
                top: Val::Px(label.screen_y),
                ..default()
            },
            bevy::picking::Pickable::IGNORE,
            GridLabel,
        ));
    }
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
