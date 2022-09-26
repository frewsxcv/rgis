use bevy::prelude::*;

pub fn system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new()
        .with_system(center_camera)
        .with_system(pan_camera_system)
        .with_system(handle_meshes_spawned_events)
        .with_system(zoom_camera_system)
}

pub fn startup_system_set() -> bevy::ecs::schedule::SystemSet {
    bevy::ecs::schedule::SystemSet::new().with_system(init_camera)
}

fn init_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
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
    set_camera_transform(&mut transform, camera_offset, camera_scale);
}

fn set_camera_transform(
    transform: &mut Transform,
    camera_offset: crate::CameraOffset,
    camera_scale: crate::CameraScale,
) {
    transform.translation = camera_offset.to_transform_translation_vec();
    transform.scale = camera_scale.to_transform_scale_vec();
    debug!("New transform scale: {:?}", transform.scale);
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
    set_camera_transform(&mut transform, camera_offset, camera_scale);
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
        let mut transform = query.single_mut();
        let bounding_rect = match projected_feature.bounding_rect {
            Some(b) => b,
            None => continue,
        };
        let layer_center = bounding_rect.center();
        let window = windows.primary();

        let canvas_size = bevy::ui::Size::new(
            f64::from(window.width() - side_panel_width.0),
            f64::from(window.height() - top_panel_height.0 - bottom_panel_height.0),
        );

        let scale = determine_scale(bounding_rect, canvas_size);
        debug!("Moving camera to look at new layer");

        let camera_scale = crate::CameraScale(scale as f32);
        let mut camera_offset = crate::CameraOffset::from_coord(layer_center);
        camera_offset.pan_x(-side_panel_width.0 / 2., camera_scale);
        camera_offset.pan_y(
            (top_panel_height.0 - bottom_panel_height.0) / 2.,
            camera_scale,
        );
        set_camera_transform(&mut transform, camera_offset, camera_scale);
    }
}

fn determine_scale(bounding_rect: geo::Rect, canvas_size: bevy::ui::Size<f64>) -> f64 {
    (bounding_rect.width() / canvas_size.width).max(bounding_rect.height() / canvas_size.height)
}
