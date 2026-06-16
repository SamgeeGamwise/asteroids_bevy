use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Physics {
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub acceleration: Vec2,
    pub drag: f32,
    pub max_speed: f32,
}

