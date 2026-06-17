use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BulletSettings {
    pub speed: f32,
    pub lifetime_seconds: f32,
    pub spawn_offset: f32,
    pub hitbox_radius: f32,
}