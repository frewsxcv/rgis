use bevy::prelude::*;

/// A single particle emitted when a feature is selected.
#[derive(Component)]
pub struct SelectionParticle {
    /// Normalised direction the particle travels in.
    pub direction: Vec2,
    /// World position where the particle was spawned.
    pub origin: Vec2,
    /// Time (seconds) since the particle was spawned.
    pub elapsed: f32,
    /// Maximum distance the particle will travel (world units).
    pub spread: f32,
    /// Initial sprite size (world units).
    pub size: f32,
}

const PARTICLE_COUNT: usize = 10;
const PARTICLE_LIFETIME: f32 = 0.5;
const PARTICLE_COLOR: Color = Color::srgba(0.0, 0.9, 1.0, 0.7);

/// Spawn a burst of particles in a ring pattern at the given world position.
pub fn spawn_burst(
    commands: &mut Commands,
    world_pos: Vec2,
    camera_scale: f32,
    asset_server: &AssetServer,
) {
    let circle: Handle<Image> = asset_server.load("circle.png");
    let spread = camera_scale * 40.0;
    let size = camera_scale * 8.0;

    for i in 0..PARTICLE_COUNT {
        let angle =
            std::f32::consts::TAU * (i as f32 / PARTICLE_COUNT as f32);
        let direction = Vec2::new(angle.cos(), angle.sin());

        commands.spawn((
            Sprite {
                color: PARTICLE_COLOR,
                image: circle.clone(),
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            Transform::from_xyz(world_pos.x, world_pos.y, 999.0),
            SelectionParticle {
                direction,
                origin: world_pos,
                elapsed: 0.0,
                spread,
                size,
            },
        ));
    }
}

/// Animate particles: move outward with quadratic ease-out, shrink, fade, and
/// despawn after their lifetime expires.
pub fn animate_selection_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut SelectionParticle)>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut sprite, mut particle) in query.iter_mut() {
        particle.elapsed += dt;

        if particle.elapsed >= PARTICLE_LIFETIME {
            commands.entity(entity).despawn();
            continue;
        }

        let t = particle.elapsed / PARTICLE_LIFETIME;
        // Quadratic ease-out: 1 - (1-t)^2
        let ease = 1.0 - (1.0 - t) * (1.0 - t);

        let offset = particle.direction * particle.spread * ease;
        transform.translation.x = particle.origin.x + offset.x;
        transform.translation.y = particle.origin.y + offset.y;

        // Shrink
        let scale = 1.0 - ease;
        sprite.custom_size = Some(Vec2::splat(particle.size * scale));

        // Fade alpha
        let alpha = 0.7 * (1.0 - t);
        sprite.color.set_alpha(alpha);
    }
}
