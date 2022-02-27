use bevy::prelude::*;

pub struct RgisCamera;

// Component that gets added to the Camera2dBundle entity.
#[derive(Component)]
pub struct Camera2d;

impl Plugin for RgisCamera {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system())
            .insert_resource(CameraScale(1.))
            .insert_resource(CameraOffset { x: 0., y: 0. })
            .add_system(center_camera.system())
            .add_system(pan_camera_system.system())
            .add_system(zoom_camera_system.system())
            .add_system(update_camera_offset.system())
            .add_system(update_camera_scale.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Camera2d);
}

pub struct CameraScale(pub f32);

pub struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

fn pan_camera_system(
    mut pan_camera_event_reader: bevy::app::EventReader<rgis_events::PanCameraEvent>,
    mut camera_offset: ResMut<CameraOffset>,
    camera_scale: ResMut<CameraScale>,
) {
    for event in pan_camera_event_reader.iter() {
        pan_x(event.x, &mut camera_offset, &camera_scale);
        pan_y(event.y, &mut camera_offset, &camera_scale);
    }
}

fn zoom_camera_system(
    mut zoom_camera_event_reader: bevy::app::EventReader<rgis_events::ZoomCameraEvent>,
    mut camera_scale: ResMut<CameraScale>,
) {
    for event in zoom_camera_event_reader.iter() {
        zoom(event.amount, &mut camera_scale)
    }
}

fn update_camera_offset(
    camera_offset: Res<CameraOffset>,
    mut camera_transform_query: Query<(&Camera2d, &mut Transform)>,
) {
    if !camera_offset.is_changed() {
        return;
    }
    debug!("Camera offset changed");
    for (_camera, mut transform) in camera_transform_query.iter_mut() {
        transform.translation = Vec3::new(camera_offset.x, camera_offset.y, 0.);
        debug!("New transform translation: {:?}", transform.translation);
    }
}

fn update_camera_scale(
    camera_scale: Res<CameraScale>,
    mut camera_transform_query: Query<(&Camera2d, &mut Transform)>,
) {
    if !camera_scale.is_changed() {
        return;
    }
    debug!("Camera scale changed");
    for (_camera, mut transform) in camera_transform_query.iter_mut() {
        transform.scale = Vec3::new(camera_scale.0, camera_scale.0, 1.);
        debug!("New transform scale: {:?}", transform.scale);
    }
}

fn pan_x(amount: f32, camera_offset: &mut CameraOffset, camera_scale: &CameraScale) {
    camera_offset.x += amount * camera_scale.0;
}

fn pan_y(amount: f32, camera_offset: &mut CameraOffset, camera_scale: &CameraScale) {
    camera_offset.y += amount * camera_scale.0;
}

fn zoom(amount: f32, camera_scale: &mut CameraScale) {
    camera_scale.0 /= amount;
}

// this should go in rgis_camera
fn center_camera(
    layers: rgis_layers::ResLayers,
    mut camera_offset: ResMut<CameraOffset>,
    mut camera_scale: ResMut<CameraScale>,
    mut event_reader: EventReader<rgis_events::CenterCameraEvent>,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let layer_center = layer.projected_bounding_rect.rect.center();
        // TODO: this scale math is inprecise. it should take into account
        // .     the height of the geometry. as well as the window size.
        let scale = layer.projected_bounding_rect.rect.width() / 1_000.;
        debug!("Moving camera to look at new layer");
        camera_offset.x = layer_center.x as f32;
        camera_offset.y = layer_center.y as f32;
        camera_scale.0 = scale as f32;
    }
}
