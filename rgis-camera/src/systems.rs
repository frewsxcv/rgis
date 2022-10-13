use bevy::prelude::*;

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
    commands.spawn().insert_bundle(Camera2dBundle::default());
}

fn handle_change_crs_event(
    mut change_crs_event_reader: bevy::ecs::event::EventReader<rgis_events::ChangeCrsEvent>,
) {
    if let Some(event) = change_crs_event_reader.iter().next_back() {}
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
    let camera_offset = crate::CameraOffset::from_transform(&transform);
    let mut camera_scale = crate::CameraScale::from_transform(&transform);
    for event in zoom_camera_event_reader.iter() {
        camera_scale.zoom(event.amount);
    }
    crate::utils::set_camera_transform(&mut transform, camera_offset, camera_scale);
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
    side_panel_width: Res<rgis_ui::SidePanelWidth>,
    top_panel_height: Res<rgis_ui::TopPanelHeight>,
    bottom_panel_height: Res<rgis_ui::BottomPanelHeight>,
) {
    for projected_feature in event_reader
        .iter()
        .filter_map(|event| layers.get(event.0))
        .filter_map(|layer| layer.get_projected_feature_or_log())
    {
        let bounding_rect = match projected_feature.bounding_rect {
            Some(b) => b,
            None => continue,
        };
        let mut transform = query.single_mut();
        let window = windows.primary();

        debug!("Moving camera to look at new layer");
        crate::utils::center_camera_on_projected_world_rect(
            crate::ProjectedWorldRect(bounding_rect),
            &mut transform,
            window,
            &side_panel_width,
            &top_panel_height,
            &bottom_panel_height,
        );
    }
}
