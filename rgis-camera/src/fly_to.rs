use bevy::prelude::*;

/// Component that drives a smooth camera fly-to animation.
#[derive(Component)]
pub struct CameraFlyTo {
    pub start_translation: Vec3,
    pub end_translation: Vec3,
    pub start_scale: Vec3,
    pub end_scale: Vec3,
    pub elapsed: f32,
    pub duration: f32,
}

impl CameraFlyTo {
    pub fn new(current_transform: &Transform, target_transform: &Transform) -> Self {
        Self {
            start_translation: current_transform.translation,
            end_translation: target_transform.translation,
            start_scale: current_transform.scale,
            end_scale: target_transform.scale,
            elapsed: 0.0,
            duration: 0.7,
        }
    }
}

pub fn fly_to_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CameraFlyTo)>,
) {
    use bevy::math::curve::{Curve, EasingCurve, EaseFunction};

    for (entity, mut transform, mut fly_to) in query.iter_mut() {
        fly_to.elapsed += time.delta_secs();
        let t = (fly_to.elapsed / fly_to.duration).clamp(0.0, 1.0);

        let easing = EasingCurve::new(0.0_f32, 1.0_f32, EaseFunction::CubicInOut);
        let eased_t = easing.sample(t).unwrap_or(t);

        transform.translation = fly_to.start_translation.lerp(fly_to.end_translation, eased_t);
        transform.scale = fly_to.start_scale.lerp(fly_to.end_scale, eased_t);

        if t >= 1.0 {
            commands.entity(entity).remove::<CameraFlyTo>();
        }
    }
}
