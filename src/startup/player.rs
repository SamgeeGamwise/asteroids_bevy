use bevy::math::Vec2;
use bevy::prelude::{Commands, KeyCode, Res, Sprite, Transform};
use crate::components::{Input, Movement, Physics};
use crate::resources::GameTextures;
use crate::components::Player;
use crate::resources::player_settings::PlayerSettings;

pub fn create_player(player: Res<PlayerSettings>, mut commands: Commands, textures: Res<GameTextures>) {
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
        }
    ));
}