#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;

pub struct Plugin;

pub struct Camera2d(pub Entity);

impl FromWorld for Camera2d {
    fn from_world(world: &mut World) -> Self {
        let entity = world
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d())
            .id();
        Camera2d(entity)
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Camera2d>()
            .add_system(center_camera)
            .add_system(pan_camera_system)
            .add_system(zoom_camera_system);
    }
}

struct CameraScale(pub f32);

struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

fn pan_camera_system(
    mut pan_camera_event_reader: bevy::ecs::event::EventReader<rgis_events::PanCameraEvent>,
    camera_2d: Res<Camera2d>,
    mut query: Query<&mut Transform>,
) {
    for event in pan_camera_event_reader.iter() {
        if let Ok(mut transform) = query.get_mut(camera_2d.0) {
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
    transform.translation = Vec3::new(
        camera_offset.x,
        camera_offset.y,
        999.9, // https://bevy-cheatbook.github.io/pitfalls/2d-camera-z.html
    );
    transform.scale = Vec3::new(camera_scale.0, camera_scale.0, 1.);
    debug!("New transform scale: {:?}", transform.scale);
}

fn zoom_camera_system(
    mut zoom_camera_event_reader: bevy::ecs::event::EventReader<rgis_events::ZoomCameraEvent>,
    camera_2d: Res<Camera2d>,
    mut query: Query<&mut Transform>,
) {
    for event in zoom_camera_event_reader.iter() {
        if let Ok(mut transform) = query.get_mut(camera_2d.0) {
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
    camera_2d: Res<Camera2d>,
    mut query: Query<&mut Transform>,
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

        if let Ok(mut transform) = query.get_mut(camera_2d.0) {
            set_camera_transform(&mut transform, &camera_offset, &camera_scale);
        }
    }
}
