use bevy::prelude::*;
use rgis_transform::Transformer;

pub fn system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new()
        .with_system(center_camera)
        .with_system(pan_camera_system)
        .with_system(handle_meshes_spawned_events)
        .with_system(zoom_camera_system)
        .with_system(handle_change_crs_event)
}

pub fn startup_system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new().with_system(init_camera)
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_change_crs_event(
    mut change_crs_event_reader: bevy::ecs::event::EventReader<rgis_events::CrsChangedEvent>,
    mut query: Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<bevy::render::camera::Camera>,
    >,
    windows: Res<bevy::window::Windows>,
    ui_margins: rgis_ui::UiMargins,
) {
    let Some(event) = change_crs_event_reader.iter().next_back() else { return };
    let mut transform = query.single_mut();
    let window = windows.primary();
    let map_area = rgis_units::MapArea {
        window,
        left_offset_px: ui_margins.left.0,
        right_offset_px: 0.,
        top_offset_px: ui_margins.top.0,
        bottom_offset_px: ui_margins.bottom.0,
    };
    let rect = map_area.projected_geo_rect(&transform, window);

    let transformer = rgis_transform::DefaultTransformer::setup(&event.old_crs, &event.new_crs);
    if let Err(e) = transformer.transform(&mut (rect.0.into())) {
        bevy::log::error!("Enountered error when transforming: {}", e);
    }

    crate::utils::center_camera_on_projected_world_rect(rect, &mut transform, map_area);
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

    for event in pan_camera_event_reader.iter() {
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
    let mut mouse_offset = camera_offset.clone();
    let before_scale = crate::CameraScale::from_transform(&transform);
    let mut camera_scale = before_scale.clone();
    let mut set = false;
    for event in zoom_camera_event_reader.iter() {
        if !set {
            set = true;
            mouse_offset = crate::CameraOffset::from_coord(event.coord.0);
        }
        camera_scale.zoom(event.amount);
    }
    if camera_scale.0.is_normal() {
        let xd = mouse_offset.x - camera_offset.x;
        let yd = mouse_offset.y - camera_offset.y;

        camera_offset.x -= xd * (1.0 - before_scale.0 / camera_scale.0);
        camera_offset.y -= yd * (1.0 - before_scale.0 / camera_scale.0);

        if camera_offset.x.is_finite() && camera_offset.y.is_finite() {
            crate::utils::set_camera_transform(&mut transform, camera_offset, camera_scale);
        }
    }
}

fn handle_meshes_spawned_events(
    mut meshes_spawned_event_reader: bevy::ecs::event::EventReader<rgis_events::MeshesSpawnedEvent>,
    mut center_camera_event_writer: bevy::ecs::event::EventWriter<rgis_events::CenterCameraEvent>,
    mut has_moved: bevy::ecs::system::Local<bool>,
) {
    for event in meshes_spawned_event_reader.iter() {
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
    windows: Res<bevy::window::Windows>,
    ui_margins: rgis_ui::UiMargins,
) {
    for projected_feature in event_reader
        .iter()
        .filter_map(|event| layers.get(event.0))
        .filter_map(|layer| layer.get_projected_feature_collection_or_log())
    {
        let Ok(bounding_rect) = projected_feature.bounding_rect() else { continue };
        let mut transform = query.single_mut();
        let window = windows.primary();

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
