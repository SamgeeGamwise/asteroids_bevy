use bevy::app::{App, Plugin, Startup};
use crate::startup::load_textures;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_textures);
    }
}