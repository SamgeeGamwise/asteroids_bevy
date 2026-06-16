use bevy::app::{App, Plugin, Startup};
use crate::startup::{create_camera, create_player, load_textures};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_textures);
    }
}