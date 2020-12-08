use bevy::prelude::*;

pub struct RgisCamera;

impl Plugin for RgisCamera {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_resource(CameraScale(1.))
            .add_resource(CameraOffset { x: 0., y: 0. })
            .add_system(process_mouse_events.system())
            .add_system(update_camera_offset.system())
            .add_system(update_camera_scale.system());
    }
}

#[derive(Debug)]
struct Camera(Entity);

fn setup(commands: &mut Commands) {
    let entity = commands
        .spawn(Camera2dBundle::default())
        .current_entity();

    commands.spawn((Camera(entity.expect("could not find entity")),));
}

const PAN_AMOUNT: f32 = 15.; // Larger number will pan more
const ZOOM_AMOUNT: f32 = 1.3; // Larger number will zoom more

pub struct CameraScale(pub f32);

pub struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

fn process_mouse_events(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_offset: ResMut<CameraOffset>,
    mut camera_scale: ResMut<CameraScale>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::Up => pan_y(PAN_AMOUNT, &mut camera_offset, &camera_scale),
            KeyCode::Right => pan_x(PAN_AMOUNT, &mut camera_offset, &camera_scale),
            KeyCode::Down => pan_y(-PAN_AMOUNT, &mut camera_offset, &camera_scale),
            KeyCode::Left => pan_x(-PAN_AMOUNT, &mut camera_offset, &camera_scale),
            KeyCode::Minus => zoom(1. / ZOOM_AMOUNT, &mut camera_scale),
            KeyCode::Equals => {
                if keyboard_input.pressed(KeyCode::RShift)
                    || keyboard_input.pressed(KeyCode::LShift)
                {
                    zoom(ZOOM_AMOUNT, &mut camera_scale);
                }
            }
            _ => {}
        }
    }
}

fn update_camera_offset(
    camera_offset: ChangedRes<CameraOffset>,
    camera_query: Query<(&Camera,)>,
    mut transform_query: Query<(&mut Transform,)>,
) {
    log::debug!("Camera offset changed");
    for (camera,) in camera_query.iter() {
        if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
            transform.translation = Vec3::new(camera_offset.x, camera_offset.y, 0.);
            log::debug!("New transform translation: {:?}", transform.translation);
        }
    }
}

fn update_camera_scale(
    camera_scale: ChangedRes<CameraScale>,
    camera_query: Query<(&Camera,)>,
    mut transform_query: Query<(&mut Transform,)>,
) {
    log::debug!("Camera scale changed");
    for (camera,) in camera_query.iter() {
        if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
            transform.scale = Vec3::new(camera_scale.0, camera_scale.0, 1.);
            log::debug!("New transform scale: {:?}", transform.scale);
        }
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
