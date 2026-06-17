use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlayerSettings {
    pub thrust: f32,
    pub turn_speed: f32,
    pub max_speed: f32,
    pub drag: f32,
    pub hitbox_radius: f32,
}