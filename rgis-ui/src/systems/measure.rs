use bevy::{ecs::query::QueryIter, prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts,
};
use geo::{Distance, Geodesic, Haversine, Rhumb};

pub(super) fn render_in_progress(
    query: Query<&bevy_jobs::InProgressJob>,
    mut bevy_egui_ctx: EguiContexts,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let mut in_progress_job_iter: std::iter::Peekable<
        QueryIter<'_, '_, &bevy_jobs::InProgressJob, ()>,
    > = query.iter().peekable();

    if in_progress_job_iter.peek().is_none() {
        return Ok(());
    }

    egui::Window::new("Running jobs")
        .open(&mut true)
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5., -5.])
        .resizable(false)
        .show(bevy_egui_ctx_mut, |ui| {
            for in_progress_job in in_progress_job_iter {
                ui.add(InProgressJobWidget { in_progress_job });
            }
        });
    Ok(())
}

struct InProgressJobWidget<'a> {
    in_progress_job: &'a bevy_jobs::InProgressJob,
}

impl Widget for InProgressJobWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { in_progress_job } = self;

        let name = &in_progress_job.name;
        let progress = in_progress_job.progress;

        ui.horizontal(|ui| {
            ui.add(egui::Spinner::new());
            if progress > 0 {
                egui::ProgressBar::new(f32::from(progress) / 100.)
                    .desired_width(200.)
                    .text(format!("Running '{name}'"))
                    .ui(ui);
            } else {
                ui.label(format!("Running '{name}'"));
            }
        })
        .response
    }
}

pub(super) struct AllDistances {
    haversine: f64,
    geodesic: f64,
    rhumb: f64,
}

fn calculate_all_distances(
    start: geo::Coord<f64>,
    end: geo::Coord<f64>,
    geodesy_ctx: &rgis_crs::GeodesyContext,
    target_crs: &rgis_crs::TargetCrs,
) -> Option<AllDistances> {
    let mut geodesy_ctx_inner = geodesy_ctx.write().ok()?;
    let target_epsg_code = 4326; // WGS 84

    let target_op_handle =
        rgis_crs::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx_inner, target_epsg_code).ok()?;

    let transformer = geo_geodesy::Transformer::from_geodesy(
        &*geodesy_ctx_inner,
        target_crs.0.op_handle,
        target_op_handle,
    )
    .ok()?;

    let mut start_lat_lon = geo::Geometry::Point(geo::Point::new(start.x, start.y));
    let mut end_lat_lon = geo::Geometry::Point(geo::Point::new(end.x, end.y));

    transformer.transform(&mut start_lat_lon).ok()?;
    transformer.transform(&mut end_lat_lon).ok()?;

    let (Some(geo::Geometry::Point(start_point)), Some(geo::Geometry::Point(end_point))) =
        (Some(start_lat_lon), Some(end_lat_lon))
    else {
        return None;
    };

    // geo_geodesy::Transformer::transform() already converts from radians to degrees
    Some(AllDistances {
        haversine: Haversine.distance(start_point, end_point),
        geodesic: Geodesic.distance(start_point, end_point),
        rhumb: Rhumb.distance(start_point, end_point),
    })
}

pub(super) fn render_measure_tool(
    mut bevy_egui_ctx: EguiContexts,
    current_tool: Res<State<rgis_settings::Tool>>,
    measure_state: Res<rgis_mouse::MeasureState>,
    mouse_pos: Res<rgis_mouse::MousePos>,
    geodesy_ctx: Res<rgis_crs::GeodesyContext>,
    target_crs: Res<rgis_crs::TargetCrs>,
    camera_q: Query<&Transform, With<Camera>>,
    windows: Query<&bevy::window::Window, With<PrimaryWindow>>,
    mut cached_distances: Local<Option<AllDistances>>,
) -> Result {
    if *current_tool.get() != rgis_settings::Tool::Measure {
        return Ok(());
    }

    let Some(start) = measure_state.start else {
        return Ok(());
    };

    // Use locked end point if set, otherwise follow cursor
    let end = measure_state.end.unwrap_or(mouse_pos.0);
    let transform = camera_q.single()?;
    let window = windows.single()?;

    // Project points to screen for rendering
    let start_screen_pos =
        project_to_screen(geo::Coord { x: start.x.0, y: start.y.0 }, transform, window);
    let end_screen_pos =
        project_to_screen(geo::Coord { x: end.x.0, y: end.y.0 }, transform, window);

    // Only recompute distances when inputs change (avoids per-frame coordinate
    // transformation and geodesic calculations)
    if measure_state.is_changed() || mouse_pos.is_changed() || target_crs.is_changed() {
        let start_coord = geo::Coord {
            x: start.x.0,
            y: start.y.0,
        };
        let end_coord = geo::Coord {
            x: end.x.0,
            y: end.y.0,
        };
        *cached_distances = calculate_all_distances(start_coord, end_coord, &geodesy_ctx, &target_crs);
    }
    let all_distances = &*cached_distances;

    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let painter = bevy_egui_ctx_mut.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("measure_tool"),
    ));

    painter.line_segment(
        [start_screen_pos, end_screen_pos],
        egui::Stroke::new(2.0, egui::Color32::RED),
    );

    // Draw drag handles at endpoints: white fill with red border
    painter.circle(start_screen_pos, 8.0, egui::Color32::WHITE, egui::Stroke::new(2.0, egui::Color32::RED));
    painter.circle(end_screen_pos, 8.0, egui::Color32::WHITE, egui::Stroke::new(2.0, egui::Color32::RED));

    // Distance panel with live distances for all methods
    let entries: &[(&str, f64, &str)] = &[
        ("Haversine", all_distances.as_ref().map_or(0.0, |d| d.haversine), "Great-circle distance using the Haversine formula"),
        ("Geodesic", all_distances.as_ref().map_or(0.0, |d| d.geodesic), "Geodesic distance on the WGS84 ellipsoid (most accurate)"),
        ("Rhumb", all_distances.as_ref().map_or(0.0, |d| d.rhumb), "Distance along a rhumb line (constant bearing)"),
    ];
    egui::Window::new("Distances")
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-8.0, -8.0])
        .resizable(false)
        .auto_sized()
        .show(bevy_egui_ctx_mut, |ui| {
            for &(name, dist, description) in entries {
                let dist_str = if dist.is_finite() {
                    crate::widgets::scale_bar::distance_to_readable_string(dist as f32)
                } else {
                    "N/A".to_string()
                };
                let label = ui.label(format!("{name}: {dist_str}")).on_hover_text(description);
                crate::widget_registry::register(name, label.rect);
            }
        });

    Ok(())
}

fn project_to_screen(
    projected: geo::Coord<f64>,
    camera_transform: &Transform,
    window: &bevy::window::Window,
) -> egui::Pos2 {
    let sc = rgis_units::ScreenCoord::from_projected(projected, camera_transform, window);
    egui::Pos2::new(sc.x as f32, sc.y as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_measure_tool() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(bevy::window::WindowPlugin::default());
        app.add_plugins(bevy::input::InputPlugin);
        // Initialize Shader asset to satisfy bevy_egui requirement
        app.init_asset::<bevy::prelude::Shader>();
        app.init_asset::<bevy::prelude::Image>();

        app.add_plugins(bevy_egui::EguiPlugin::default());
        app.add_plugins(rgis_events::RgisEventsPlugin);
        app.add_plugins(rgis_crs::Plugin::default());
        app.add_plugins(bevy::state::app::StatesPlugin);

        app.insert_state(rgis_settings::Tool::Measure);
        app.insert_resource(rgis_settings::RgisSettings {
            show_scale: true,
            dark_mode: false,
        });

        app.insert_resource(rgis_mouse::MeasureState {
            start: Some(geo::Coord {
                x: 0.0.into(),
                y: 0.0.into(),
            }),
            end: None,
            dragging: None,
        });

        app.insert_resource(rgis_mouse::MousePos(geo::Coord {
            x: 10.0.into(),
            y: 10.0.into(),
        }));

        // Spawn an entity with Transform and Camera, which is what the system queries for.
        // We avoid using Camera2d bundle/component to avoid pulling in too many render dependencies.
        app.world_mut().spawn((
            Transform::default(),
            Camera::default(),
        ));

        app.update();

        app.add_systems(bevy_egui::EguiPrimaryContextPass, render_measure_tool);
        app.update();
    }

    #[test]
    fn test_project_to_screen() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::window::WindowPlugin::default());

        let window_entity = app
            .world_mut()
            .spawn(bevy::window::Window {
                resolution: bevy::window::WindowResolution::new(800, 600),
                ..default()
            })
            .id();

        app.update();

        let window = app.world().get::<bevy::window::Window>(window_entity).unwrap();
        let camera_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let projected = geo::Coord { x: 0.0, y: 0.0 };

        let screen_pos = super::project_to_screen(projected, &camera_transform, window);

        assert_eq!(screen_pos.x, 400.0);
        assert_eq!(screen_pos.y, 300.0);
    }

    #[test]
    fn test_calculate_all_distances() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(rgis_events::RgisEventsPlugin);
        app.add_plugins(rgis_crs::Plugin::default());

        app.update();

        // Manually set TargetCrs to 4326 (WGS84) to simplify test and verify logic without projection issues
        let op_handle = {
            let geodesy_ctx = app.world().resource::<rgis_crs::GeodesyContext>();
            let mut geodesy_ctx_inner = geodesy_ctx.write().unwrap();
            rgis_crs::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx_inner, 4326).unwrap()
        };

        app.insert_resource(rgis_crs::TargetCrs(rgis_primitives::Crs {
            epsg_code: Some(4326),
            proj_string: None,
            op_handle,
        }));

        let geodesy_ctx = app.world().resource::<rgis_crs::GeodesyContext>();
        let target_crs = app.world().resource::<rgis_crs::TargetCrs>();

        // San Francisco (lon: -122.4194, lat: 37.7749)
        let start = geo::Coord {
            x: -122.4194,
            y: 37.7749,
        };
        // New York City (lon: -74.0060, lat: 40.7128)
        let end = geo::Coord {
            x: -74.0060,
            y: 40.7128,
        };

        let distances =
            super::calculate_all_distances(start, end, geodesy_ctx, target_crs).unwrap();

        // All distances must be finite (regression: a double .to_degrees() conversion
        // previously caused Geodesic to return NaN)
        assert!(distances.haversine.is_finite(), "Haversine was {}", distances.haversine);
        assert!(distances.geodesic.is_finite(), "Geodesic was {}", distances.geodesic);
        assert!(distances.rhumb.is_finite(), "Rhumb was {}", distances.rhumb);

        // Haversine distance is approx 4,129 km
        assert!(
            distances.haversine > 4_120_000.0 && distances.haversine < 4_140_000.0,
            "Haversine distance was {}",
            distances.haversine
        );

        // Geodesic distance is approx 4,139 km
        assert!(
            distances.geodesic > 4_130_000.0 && distances.geodesic < 4_150_000.0,
            "Geodesic distance was {}",
            distances.geodesic
        );

        // Rhumb line distance SF-NYC is longer than great-circle, approx 4,181 km
        assert!(
            distances.rhumb > 4_170_000.0 && distances.rhumb < 4_200_000.0,
            "Rhumb distance was {}",
            distances.rhumb
        );

    }

    /// Regression test: geo_geodesy::Transformer::transform() already converts
    /// output from radians to degrees. A previous bug called .to_degrees() again,
    /// turning valid coordinates like (-122, 37) into (-6692, 2282). This
    /// caused Geodesic to return NaN and Rhumb to return wildly wrong values.
    #[test]
    fn test_double_degrees_conversion_causes_geodesic_nan() {
        use geo::Distance;

        // These are the coordinates produced by the double .to_degrees() bug:
        // e.g. (-122.4194).to_degrees() = -6692.4
        let start = geo::Point::new(-6692.4, 2282.0);
        let end = geo::Point::new(-4395.1, 2552.0);

        // Geodesic returns NaN for these out-of-range coordinates
        assert!(
            Geodesic.distance(start, end).is_nan(),
            "Geodesic should return NaN for out-of-range coordinates"
        );

        // The correct coordinates (in degrees) should produce finite results
        let sf = geo::Point::new(-122.4194, 37.7749);
        let nyc = geo::Point::new(-74.0060, 40.7128);
        let geodesic_dist = Geodesic.distance(sf, nyc);
        assert!(
            geodesic_dist.is_finite() && geodesic_dist > 4_000_000.0,
            "Geodesic distance for valid coordinates was {}",
            geodesic_dist
        );
    }
}
