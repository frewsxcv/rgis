#![warn(clippy::unwrap_used, clippy::cast_lossless, clippy::unimplemented, clippy::expect_used)]

use bevy::prelude::*;

pub struct Plugin;

// Component that gets added to the Camera2dBundle entity.
#[derive(Component)]
pub struct Camera2d;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system())
            .add_system(center_camera.system())
            .add_system(pan_camera_system.system())
            .add_system(zoom_camera_system.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Camera2d);
}

struct CameraScale(pub f32);

struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

fn pan_camera_system(
    mut pan_camera_event_reader: bevy::app::EventReader<rgis_events::PanCameraEvent>,
    mut camera_transform_query: Query<&mut Transform, With<Camera2d>>,
) {
    for event in pan_camera_event_reader.iter() {
        for mut transform in camera_transform_query.iter_mut() {
            let mut camera_offset = CameraOffset {
                x: transform.translation[0],
                y: transform.translation[1],
            };
            let camera_scale = CameraScale(transform.scale[0]);

            pan_x(event.x, &mut camera_offset, &camera_scale);
            pan_y(event.y, &mut camera_offset, &camera_scale);

            set_camera_transform(&mut transform, &camera_offset, &camera_scale);
        }
    }
}

fn set_camera_transform(
    transform: &mut Transform,
    camera_offset: &CameraOffset,
    camera_scale: &CameraScale,
) {
    transform.translation = Vec3::new(camera_offset.x, camera_offset.y, 0.);
    transform.scale = Vec3::new(camera_scale.0, camera_scale.0, 1.);
    debug!("New transform scale: {:?}", transform.scale);
}

fn zoom_camera_system(
    mut zoom_camera_event_reader: bevy::app::EventReader<rgis_events::ZoomCameraEvent>,
    mut camera_transform_query: Query<&mut Transform, With<Camera2d>>,
) {
    for event in zoom_camera_event_reader.iter() {
        for mut transform in camera_transform_query.iter_mut() {
            let camera_offset = CameraOffset {
                x: transform.translation[0],
                y: transform.translation[1],
            };
            let mut camera_scale = CameraScale(transform.scale[0]);

            zoom(event.amount, &mut camera_scale);

            set_camera_transform(&mut transform, &camera_offset, &camera_scale);
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

fn center_camera(
    layers: Res<rgis_layers::Layers>,
    mut event_reader: EventReader<rgis_events::CenterCameraEvent>,
    mut camera_transform_query: Query<&mut Transform, With<Camera2d>>,
) {
    for event in event_reader.iter() {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let layer_center = layer.projected_bounding_rect.center();
        // TODO: this scale math is inprecise. it should take into account
        // .     the height of the geometry. as well as the window size.
        let scale = layer.projected_bounding_rect.width() / 1_000.;
        debug!("Moving camera to look at new layer");
        let camera_offset = CameraOffset {
            x: layer_center.x as f32,
            y: layer_center.y as f32,
        };
        let camera_scale = CameraScale(scale as f32);

        for mut transform in camera_transform_query.iter_mut() {
            set_camera_transform(&mut transform, &camera_offset, &camera_scale);
        }
    }
}
