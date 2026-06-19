use crate::asteroid::components::{Asteroid, AsteroidSize, InvulnerabilityTimer};
use crate::physics::Physics;
use crate::settings::{AsteroidSettings, WorldSettings};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::*;
use rand::RngExt;

pub fn create_asteroid(
    asteroid: &AsteroidSettings,
    world: &WorldSettings,
    commands: &mut Commands,
    size: AsteroidSize,
    asteroid_texture: Handle<Image>,
    translation_option: Option<Vec3>,
) {
    let mut rng = rand::rng();

    let translation = match translation_option {
        Some(translation) => Vec3::new(
            translation.x + rng.random_range(-5.0..5.0),
            translation.y + rng.random_range(-5.0..5.0),
            0.0,
        ),
        None => get_random_translation(world),
    };

    let transform = Transform {
        translation,
        scale: Vec3::ONE * asteroid.scale,
        rotation: Quat::from_rotation_z(0.0),
    };

    commands.spawn((
        Asteroid::new(size),
        transform,
        Sprite::from_image(asteroid_texture),
        Physics {
            velocity: get_random_trajectory(asteroid),
            angular_velocity: get_random_angular_velocity(asteroid),
            acceleration: Vec2::ZERO,
            drag: 0.0,
            max_speed: asteroid.max_speed,
        },
        InvulnerabilityTimer::new(asteroid.invulnerability_timer),
    ));
}

fn get_random_trajectory(asteroid: &AsteroidSettings) -> Vec2 {
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

    Vec2::new(velocity_x, velocity_y) * rng.random_range(0.1..=asteroid.speed)
}

fn get_random_translation(world: &WorldSettings) -> Vec3 {
    let mut rng = rand::rng();
    let top_bottom = rng.random_bool(0.5);
    let start_or_finish = rng.random_bool(0.5);
    let top = -world.virtual_height / 2.0 - world.wrap_buffer;
    let bottom = world.virtual_height / 2.0 + world.wrap_buffer;
    let left = -world.virtual_width / 2.0 - world.wrap_buffer;
    let right = world.virtual_width / 2.0 + world.wrap_buffer;

    let (x, y) = if top_bottom {
        let y = rng.random_range(top..bottom);
        let x = if start_or_finish { left } else { right };
        (x, y)
    } else {
        let x = rng.random_range(left..right);
        let y = if start_or_finish { top } else { bottom };
        (x, y)
    };

    Vec3::new(x, y, 0.0)
}

fn get_random_angular_velocity(asteroid: &AsteroidSettings) -> f32 {
    let mut rng = rand::rng();
    rng.random::<f32>() * asteroid.angular_speed
}
