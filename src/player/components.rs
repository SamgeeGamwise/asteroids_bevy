use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Input {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub fire: KeyCode,
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub turn_speed: f32,
}
