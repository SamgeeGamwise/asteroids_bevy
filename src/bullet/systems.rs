use crate::bullet::components::{Bullet, LifeTime};
use crate::physics::Physics;
use crate::settings::BulletSettings;
use bevy::math::{Quat, Vec2};
use bevy::prelude::*;

pub fn create_bullet(
    bullet: &BulletSettings,
    commands: &mut Commands,
    bullet_texture: Handle<Image>,
    player_transform: &Transform,
) {
    let direction = player_transform.up().truncate().normalize_or_zero();

    let spawn_position =
        player_transform.translation + (direction * bullet.spawn_offset).extend(0.0);

    let bullet_transform = Transform::from_translation(spawn_position).with_rotation(
        player_transform.rotation * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
    );

    commands.spawn((
        Bullet,
        bullet_transform,
        Sprite::from_image(bullet_texture),
        Physics {
            velocity: direction * bullet.speed,
            angular_velocity: 0.0,
            acceleration: Vec2::ZERO,
            drag: 0.0,
            max_speed: bullet.speed,
        },
        LifeTime::new(bullet.lifetime_seconds),
    ));
}

pub fn despawn_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
