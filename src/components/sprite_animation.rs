use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Component, Res, Timer, TimerMode};
use crate::resources::GameTextures;

#[derive(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub frames: Vec<Handle<Image>>,
    pub current_frame: usize,
}

impl SpriteAnimation {
    pub fn new(time: f32, textures: Vec<Handle<Image>>) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
            frames: textures,
            current_frame: 0,
        }
    }
}
