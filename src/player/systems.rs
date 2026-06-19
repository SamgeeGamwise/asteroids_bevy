use crate::assets::GameTextures;
use crate::bullet::systems::create_bullet;
use crate::physics::Physics;
use crate::player::components::{Input, Movement, Player};
use crate::settings::{BulletSettings, PlayerSettings};
use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::*;

pub fn handle_global_input(keyboard: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn create_player(
    player: Res<PlayerSettings>,
    mut commands: Commands,
    textures: Res<GameTextures>,
) {
    commands.spawn((
        Player,
        Transform::default(),
        Sprite::from_image(textures.player_ship.clone()),
        Physics {
            velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            acceleration: Vec2::ZERO,
            drag: player.drag,
            max_speed: player.max_speed,
        },
        Movement {
            speed: player.thrust,
            turn_speed: player.turn_speed,
        },
        Input {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            fire: KeyCode::Space,
        },
    ));
}

pub fn handle_player_movement(
    mut query: Query<(&Movement, &mut Physics, &Input, &mut Transform), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (movement, mut physics, input, mut transform) in &mut query {
        physics.acceleration = Vec2::ZERO;

        let forward = transform.up().truncate();

        if keyboard.pressed(input.move_forward) {
            physics.acceleration += forward * movement.speed;
        } else if keyboard.pressed(input.move_backward) {
            physics.acceleration -= forward * movement.speed;
        }

        if keyboard.pressed(input.move_left) {
            transform.rotate_z(movement.turn_speed * time.delta_secs());
        } else if keyboard.pressed(input.move_right) {
            transform.rotate_z(-movement.turn_speed * time.delta_secs());
        }
    }
}

pub fn handle_player_fire_bullet(
    mut commands: Commands,
    textures: Res<GameTextures>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &Input), With<Player>>,
    bullet: Res<BulletSettings>,
) {
    for (transform, input) in &mut query {
        if keyboard.just_pressed(input.fire) {
            create_bullet(&bullet, &mut commands, textures.bullet.clone(), transform);
        }
    }
}
