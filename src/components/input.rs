use bevy::prelude::{Component, KeyCode};

#[derive(Component)]
pub struct Input {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub fire: KeyCode,
}
