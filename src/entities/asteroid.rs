use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Commands, Sprite, Timer, TimerMode, Transform};
use bevy::transform;
use crate::components::{Bullet, LifeTime, Movement, Physics};
use crate::resources::GameTextures;
use rand::{Rng, RngExt};
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, WRAP_BUFFER};
use crate::components::asteroid::{Asteroid, AsteroidSize};

const ASTEROID_SPEED: f32 = 500.0;

pub fn create_asteroid(
    commands: &mut Commands,
    size: AsteroidSize,
    asteroid_texture: Handle<Image>
) {
    commands.spawn((
        Asteroid::new(size),
        get_random_transform(size),
        Sprite::from_image(asteroid_texture),
        Physics {
            velocity: get_random_trajectory(),
            angular_velocity: get_random_angular_velocity(),
            acceleration: Vec2::ZERO,
            drag: 0.0,
            max_speed: 200.0,
        }
    ));
}

fn get_random_trajectory() -> Vec2 {
    let mut rng = rand::rng();

    let velocity_x: f32 = if rng.random_bool(0.5) {
        rng.random::<f32>()
    } else {
        -rng.random::<f32>()
    };

    let velocity_y: f32 = if rng.random_bool(0.5) {
        rng.random::<f32>()
    } else {
        -rng.random::<f32>()
    };

    Vec2::new(velocity_x, velocity_y) * rng.random_range(0.1..=ASTEROID_SPEED)
}

fn get_random_transform(size: AsteroidSize) -> Transform {
    let mut rng = rand::rng();
    let top_bottom = rng.random_bool(0.5);
    let start_or_finish = rng.random_bool(0.5);
    let x: f32;
    let y: f32;
    let top = -VIRTUAL_HEIGHT / 2.0 - WRAP_BUFFER;
    let bottom = VIRTUAL_HEIGHT / 2.0 + WRAP_BUFFER;
    let left = -VIRTUAL_WIDTH / 2.0 - WRAP_BUFFER;
    let right = VIRTUAL_WIDTH / 2.0 + WRAP_BUFFER;

    if top_bottom {
        y = rng.random_range(top..bottom);

        if start_or_finish {
            x = left;
        } else {
            x = right;
        }
    } else {
        x = rng.random_range(left..right);

        if start_or_finish {
            y = top;
        } else {
            y = bottom;
        }
    }

    let scale = match size {
        AsteroidSize::Large => 2.0,
        AsteroidSize::Medium => 1.0,
        AsteroidSize::Small => 0.75,
    };

    let transform = Transform {
        translation: Vec3::new(x, y, 0.0),
        scale: Vec3::ONE * scale,
        rotation: Quat::from_rotation_z(0.0),
    };

    transform
}

fn get_random_angular_velocity() -> f32 {
    let mut rng = rand::rng();
    rng.random()
}