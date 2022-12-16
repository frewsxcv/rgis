#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod systems;
mod utils;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(systems::startup_system_set())
            .add_system_set(systems::system_set());
    }
}

#[derive(Clone, Copy)]
struct CameraScale(pub f32);

impl CameraScale {
    fn from_transform(transform: &Transform) -> Self {
        CameraScale(transform.scale.as_ref()[0])
    }

    fn zoom(&mut self, amount: f32) {
        self.0 /= amount;
    }

    fn to_transform_scale_vec(self) -> Vec3 {
        Vec3::new(self.0, self.0, 1.)
    }
}

#[derive(Clone, Copy)]
struct CameraOffset {
    /// Units: world coordinates
    pub x: f32,
    /// Units: world coordinates
    pub y: f32,
}

impl CameraOffset {
    fn from_coord(coord: geo::Coord) -> Self {
        CameraOffset {
            x: coord.x as f32,
            y: coord.y as f32,
        }
    }

    fn from_transform(transform: &Transform) -> Self {
        CameraOffset {
            x: transform.translation.as_ref()[0],
            y: transform.translation.as_ref()[1],
        }
    }

    fn pan_x(&mut self, amount: f32, camera_scale: CameraScale) {
        // what is the camera scale?
        self.x += amount * camera_scale.0;
    }

    fn pan_y(&mut self, amount: f32, camera_scale: CameraScale) {
        self.y += amount * camera_scale.0;
    }

    fn to_transform_translation_vec(self) -> Vec3 {
        Vec3::new(
            self.x, self.y,
            999.9, // https://bevy-cheatbook.github.io/pitfalls/2d-camera-z.html
        )
    }
}
