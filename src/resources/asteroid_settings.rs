use bevy::prelude::Resource;

#[derive(Resource)]
pub struct AsteroidSettings {
    pub spawn_timer: f32,
    pub invulnerability_timer: f32,
    pub scale: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub angular_speed: f32,
    pub hitbox_radius_large: f32,
    pub hitbox_radius_medium: f32,
    pub hitbox_radius_small: f32,
}