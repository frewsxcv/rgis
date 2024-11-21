use bevy::{prelude::*, window::PrimaryWindow};

pub fn configure(app: &mut App) {
    app.add_systems(Startup, init_camera);
    app.add_systems(
        Update,
        (
            center_camera,
            pan_camera_system,
            handle_meshes_spawned_events,
            zoom_camera_system,
            handle_change_crs_event.pipe(log_error),
        ),
    );
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn log_error(result: In<Result<(), Box<dyn std::error::Error + Send + Sync>>>) {
    if let Err(e) = result.0 {
        bevy::log::error!("{}", e);
    }
}

fn handle_change_crs_event(
    mut change_crs_event_reader: bevy::ecs::event::EventReader<rgis_events::CrsChangedEvent>,
    mut query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
    ui_margins: rgis_units::UiMargins,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Some(event) = change_crs_event_reader.read().last() else {
        return Ok(());
    };
    let window = windows.get_single()?;
    let mut transform = query.single_mut();
    let map_area = rgis_units::MapArea {
        window,
        left_offset_px: ui_margins.left.0,
        right_offset_px: 0.,
        top_offset_px: ui_margins.top.0,
        bottom_offset_px: ui_margins.bottom.0,
    };
    let rect = map_area.projected_geo_rect(&transform, window);

    let transformer =
        transform::Transformer::setup(event.old_crs_epsg_code, event.new_crs_epsg_code)?;
    transformer.transform(&mut (rect.into()))?;

    crate::utils::center_camera_on_projected_world_rect(rect, &mut transform, map_area);

    Ok(())
}

fn pan_camera_system(
    mut pan_camera_event_reader: bevy::ecs::event::EventReader<rgis_events::PanCameraEvent>,
    mut query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
) {
    if pan_camera_event_reader.is_empty() {
        return;
    }
    let mut transform = query.single_mut();
    let mut camera_offset = crate::CameraOffset::from_transform(&transform);
    let camera_scale = crate::CameraScale::from_transform(&transform);

    for event in pan_camera_event_reader.read() {
        camera_offset.pan_x(event.x, camera_scale);
        camera_offset.pan_y(event.y, camera_scale);
    }
    crate::utils::set_camera_transform(&mut transform, camera_offset, camera_scale);
}

fn zoom_camera_system(
    mut zoom_camera_event_reader: bevy::ecs::event::EventReader<rgis_events::ZoomCameraEvent>,
    mut query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
) {
    if zoom_camera_event_reader.is_empty() {
        return;
    }
    let mut transform = query.single_mut();
    let mut camera_offset = crate::CameraOffset::from_transform(&transform);
    let mut mouse_offset = camera_offset;
    let before_scale = crate::CameraScale::from_transform(&transform);
    let mut camera_scale = before_scale;
    let mut set = false;
    for event in zoom_camera_event_reader.read() {
        // Set mouse_offset based on the first event's coordinate
        if !set {
            set = true;
            mouse_offset = match crate::CameraOffset::from_coord(event.coord) {
                Ok(offset) => offset,
                Err(e) => {
                    error!("Error creating camera offset: {:?}", e);
                    continue;
                }
            };
        }
        // Adjust the camera scale based on the zoom amount from the event
        camera_scale.zoom(event.amount);
    }

    // Check if the new camera scale is a normal floating-point number
    if camera_scale.0.is_normal() {
        // Calculate the difference between mouse and camera offsets
        let xd = mouse_offset.x - camera_offset.x;
        let yd = mouse_offset.y - camera_offset.y;

        // Adjust the camera offset based on the scale change
        camera_offset.x -= xd * (1.0 - before_scale.0 / camera_scale.0);
        camera_offset.y -= yd * (1.0 - before_scale.0 / camera_scale.0);

        // Ensure the new camera offset values are finite before applying the transform
        if camera_offset.x.is_finite() && camera_offset.y.is_finite() {
            // Update the camera transform with the new offset and scale
            crate::utils::set_camera_transform(&mut transform, camera_offset, camera_scale);
        }
    }
}

fn handle_meshes_spawned_events(
    mut meshes_spawned_event_reader: bevy::ecs::event::EventReader<rgis_events::MeshesSpawnedEvent>,
    mut center_camera_event_writer: bevy::ecs::event::EventWriter<rgis_events::CenterCameraEvent>,
    mut has_moved: bevy::ecs::system::Local<bool>,
) {
    for event in meshes_spawned_event_reader.read() {
        if !(*has_moved) {
            center_camera_event_writer.send(event.0.into());
            *has_moved = true;
        }
    }
}

fn center_camera(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::CenterCameraEvent>,
    mut query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
    ui_margins: rgis_units::UiMargins,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    for projected_feature in event_reader
        .read()
        .filter_map(|event| layers.get(event.0))
        .filter_map(|layer| layer.get_projected_feature_collection_or_log())
    {
        let Ok(bounding_rect) = projected_feature.bounding_rect() else {
            continue;
        };
        let mut transform = query.single_mut();

        debug!("Moving camera to look at new layer");
        let map_area = rgis_units::MapArea {
            window,
            right_offset_px: 0.,
            left_offset_px: ui_margins.left.0,
            bottom_offset_px: ui_margins.bottom.0,
            top_offset_px: ui_margins.top.0,
        };
        crate::utils::center_camera_on_projected_world_rect(
            bounding_rect,
            &mut transform,
            map_area,
        );
    }
}
