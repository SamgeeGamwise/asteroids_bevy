use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, KeyCode, MessageWriter, Query, Res, Time, Transform, With};
use crate::components::{ Input, Movement, Physics};
use crate::resources::GameTextures;
use crate::components::Player;
use crate::entities::bullet::create_bullet;

pub fn handle_global_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn handle_player_movement(
    mut query: Query<(&Movement, &mut Physics, &Input, &mut Transform), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
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
) {
    for (transform, input) in &mut query {
        if keyboard.just_pressed(input.fire) {
            create_bullet(
                &mut commands,
                textures.bullet.clone(),
                transform
            );
        }
    }
}
