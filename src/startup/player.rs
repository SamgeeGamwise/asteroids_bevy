use bevy::math::Vec2;
use bevy::prelude::{Commands, KeyCode, Res, Sprite, Transform};
use crate::{GameTextures, Input, Movement, Physics, Player};

pub fn create_player(mut commands: Commands, textures: Res<GameTextures>) {
    commands.spawn((
        Player,
        Transform::default(),
        Sprite::from_image(textures.player_ship.clone()),
        Physics {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            drag: 1.0,
        },
        Movement {
            speed: 1200.0,
            turn_speed: 2.0,
            max_speed: 500.0,
        },
        Input {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            fire: KeyCode::Space,
        })
    );
}