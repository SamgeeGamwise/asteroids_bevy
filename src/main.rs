pub mod startup;
pub mod entities;
pub mod systems;
pub mod components;
pub mod resources;
pub mod plugins;

use crate::startup::{create_camera, create_player, load_textures };
use crate::systems::{ handle_player_movement, handle_player_fire_bullet, handle_global_input, handle_physics, screen_wrap, despawn_bullets, detect_collisions, AsteroidAsteroidCollision, PlayerAsteroidCollision };
use bevy::prelude::*;
use crate::plugins::game_plugin_group::GamePluginGroup;
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::systems::asteroid_spawner::spawn_asteroid;

const VIRTUAL_WIDTH: f32 = 1920.0;
const VIRTUAL_HEIGHT: f32 = 1080.0;
const WRAP_BUFFER: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePluginGroup)
        .run();
}