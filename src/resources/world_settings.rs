use bevy::prelude::Resource;

#[derive(Resource)]
pub struct WorldSettings {
    pub virtual_width: f32,
    pub virtual_height: f32,
    pub wrap_buffer: f32,
}