use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ExplosionSettings {
    pub frame_per_time: f32
}