use bevy::math::{Quat, Vec2};
use bevy::prelude::{Commands, Sprite, Timer, TimerMode, Transform};
use crate::{Bullet, GameTextures, LifeTime, Movement, Physics};

pub fn create_bullet(commands: &mut Commands, textures: &GameTextures, player_transform: &Transform) {
    let bullet_speed = 800.0;
    let direction = player_transform.up().truncate().normalize_or_zero();
    let spawn_offset = 60.0;

    let spawn_position =
        player_transform.translation + (direction * spawn_offset).extend(0.0);

    let bullet_transform = Transform::from_translation(spawn_position)
        .with_rotation(
            player_transform.rotation
                * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
        );

    commands.spawn((
        Bullet,
        bullet_transform,
        Sprite::from_image(textures.bullet.clone()),
        Physics {
            velocity: direction * bullet_speed,
            acceleration: Vec2::ZERO,
            drag: 0.0,
        },
        Movement {
            speed: 0.0,
            turn_speed: 0.0,
            max_speed: bullet_speed,
        },
        LifeTime {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    ));
}