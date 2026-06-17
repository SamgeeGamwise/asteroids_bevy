pub mod startup;
pub mod entities;
pub mod systems;
pub mod components;
pub mod resources;
pub mod plugins;
pub mod events;

use bevy::prelude::*;
use crate::plugins::game_plugin_group::GamePluginGroup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePluginGroup)
        .run();
}