pub mod startup;
pub mod entities;
pub mod systems;
pub mod components;
pub mod resources;
pub mod plugins;

use bevy::prelude::*;
use crate::plugins::game_plugin_group::GamePluginGroup;

const VIRTUAL_WIDTH: f32 = 1920.0;
const VIRTUAL_HEIGHT: f32 = 1080.0;
const WRAP_BUFFER: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePluginGroup)
        .run();
}