use bevy::prelude::Component;

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub turn_speed: f32,
}
