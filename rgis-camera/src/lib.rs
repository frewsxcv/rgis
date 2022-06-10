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
            .add_system(handle_meshes_spawned_events)
            .add_system(zoom_camera_system);
    }
}

#[derive(Clone, Copy)]
struct CameraScale(pub f32);

impl CameraScale {
    fn to_transform_scale_vec(self) -> Vec3 {
        Vec3::new(self.0, self.0, 1.)
    }
}

#[derive(Clone, Copy)]
struct CameraOffset {
    pub x: f32,
    pub y: f32,
}

impl CameraOffset {
    fn to_transform_translation_vec(self) -> Vec3 {
        Vec3::new(
            self.x, self.y,
            999.9, // https://bevy-cheatbook.github.io/pitfalls/2d-camera-z.html
        )
    }
}

fn pan_camera_system(
    mut pan_camera_event_reader: bevy::ecs::event::EventReader<rgis_events::PanCameraEvent>,
    camera_2d: Res<Camera2d>,
    mut query: Query<&mut Transform>,
) {
    if pan_camera_event_reader.is_empty() {
        return;
    }
    let mut transform = match query.get_mut(camera_2d.0) {
        Ok(t) => t,
        Err(_) => return,
    };
    let mut camera_offset = CameraOffset {
        x: transform.translation[0],
        y: transform.translation[1],
    };
    let camera_scale = CameraScale(transform.scale[0]);

    for event in pan_camera_event_reader.iter() {
        pan_x(event.x, &mut camera_offset, camera_scale);
        pan_y(event.y, &mut camera_offset, camera_scale);
        set_camera_transform(&mut transform, camera_offset, camera_scale);
    }
}

fn set_camera_transform(
    transform: &mut Transform,
    camera_offset: CameraOffset,
    camera_scale: CameraScale,
) {
    transform.translation = camera_offset.to_transform_translation_vec();
    transform.scale = camera_scale.to_transform_scale_vec();
    debug!("New transform scale: {:?}", transform.scale);
}

fn zoom_camera_system(
    mut zoom_camera_event_reader: bevy::ecs::event::EventReader<rgis_events::ZoomCameraEvent>,
    camera_2d: Res<Camera2d>,
    mut query: Query<&mut Transform>,
) {
    if zoom_camera_event_reader.is_empty() {
        return;
    }
    let mut transform = match query.get_mut(camera_2d.0) {
        Ok(t) => t,
        Err(_) => return,
    };
    let camera_offset = CameraOffset {
        x: transform.translation[0],
        y: transform.translation[1],
    };
    let mut camera_scale = CameraScale(transform.scale[0]);
    for event in zoom_camera_event_reader.iter() {
        zoom(event.amount, &mut camera_scale);

        set_camera_transform(&mut transform, camera_offset, camera_scale);
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

fn pan_x(amount: f32, camera_offset: &mut CameraOffset, camera_scale: CameraScale) {
    // what is the camera scale?
    camera_offset.x += amount * camera_scale.0;
}

fn pan_y(amount: f32, camera_offset: &mut CameraOffset, camera_scale: CameraScale) {
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
    windows: Res<bevy::window::Windows>,
) {
    for layer in event_reader.iter().filter_map(|event| layers.get(event.0)) {
        let mut transform = match query.get_mut(camera_2d.0) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let layer_center = layer.projected_bounding_rect.center();
        let window = windows.primary();
        // TODO: this should subtract the topbar, sidebar, and bottombar sizes.
        let scale = (layer.projected_bounding_rect.width() / f64::from(window.width()))
            .max(layer.projected_bounding_rect.height() / f64::from(window.height()));
        debug!("Moving camera to look at new layer");
        let camera_offset = CameraOffset {
            x: layer_center.x as f32,
            y: layer_center.y as f32,
        };
        let camera_scale = CameraScale(scale as f32);
        set_camera_transform(&mut transform, camera_offset, camera_scale);
    }
}
