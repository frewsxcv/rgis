use bevy::prelude::*;

const ZOOM_AMOUNT: f32 = 1.3; // Larger number will zoom more

pub struct RgisCamera;

impl Plugin for RgisCamera {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_event::<PanCameraEvent>()
            .add_event::<ZoomCameraEvent>()
            .add_resource(CameraScale(1.))
            .add_resource(CameraOffset { x: 0., y: 0. })
            .add_system(pan_camera_system.system())
            .add_system(zoom_camera_system.system())
            .add_system(update_camera_offset.system())
            .add_system(update_camera_scale.system());
    }
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct CameraScale(pub f32);

pub struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct PanCameraEvent {
    // X offset for camera position. Positive is right, negative is left.
    pub x: f32,
    // Y offset for camera position. Positive is up, negative is down.
    pub y: f32,
}

impl PanCameraEvent {
    pub fn up(amount: f32) -> Self {
        PanCameraEvent {
            x: 0.,
            y: amount,
        }
    }

    pub fn right(amount: f32) -> Self {
        PanCameraEvent {
            x: amount,
            y: 0.,
        }
    }

    pub fn down(amount: f32) -> Self {
        PanCameraEvent {
            x: 0.,
            y: -amount,
        }
    }

    pub fn left(amount: f32) -> Self {
        PanCameraEvent {
            x: -amount,
            y: 0.,
        }
    }
}

#[derive(Debug)]
pub struct ZoomCameraEvent {
    // (amount ∈ (1, ∞)) → zoom in
    // (amount ∈ [1] → no change
    // (amount ∈ (0, 1)) → zoom out
    amount: f32,
}

impl ZoomCameraEvent {
    pub fn zoom_in() -> Self {
        ZoomCameraEvent { amount: ZOOM_AMOUNT }
    }

    pub fn zoom_out() -> Self {
        ZoomCameraEvent { amount: 1. / ZOOM_AMOUNT }
    }
}

fn pan_camera_system(
    mut pan_camera_event_reader: bevy::ecs::Local<
        bevy::app::EventReader<PanCameraEvent>,
    >,
    pan_camera_events: bevy::ecs::Res<bevy::app::Events<PanCameraEvent>>,
    mut camera_offset: ResMut<CameraOffset>,
    mut camera_scale: ResMut<CameraScale>,
) {
    for event in pan_camera_event_reader.iter(&pan_camera_events) {
        pan_x(event.x, &mut camera_offset, &mut camera_scale);
        pan_y(event.y, &mut camera_offset, &mut camera_scale);
    }
}

fn zoom_camera_system(
    mut zoom_camera_event_reader: bevy::ecs::Local<
        bevy::app::EventReader<ZoomCameraEvent>,
    >,
    zoom_camera_events: bevy::ecs::Res<bevy::app::Events<ZoomCameraEvent>>,
    mut camera_scale: ResMut<CameraScale>,
) {
    for event in zoom_camera_event_reader.iter(&zoom_camera_events) {
        zoom(event.amount, &mut camera_scale)
    }
}

fn update_camera_offset(
    camera_offset: ChangedRes<CameraOffset>,
    mut camera_transform_query: Query<(&bevy::render::camera::Camera, &mut Transform)>,
) {
    debug!("Camera offset changed");
    for (_camera, mut transform) in camera_transform_query.iter_mut() {
        transform.translation = Vec3::new(camera_offset.x, camera_offset.y, 0.);
        debug!("New transform translation: {:?}", transform.translation);
    }
}

fn update_camera_scale(
    camera_scale: ChangedRes<CameraScale>,
    mut camera_transform_query: Query<(&bevy::render::camera::Camera, &mut Transform)>,
) {
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
