pub mod startup;
pub mod entities;
pub mod systems;
pub mod components;
pub mod resources;
pub mod plugins;

use crate::startup::{create_camera, create_player, load_textures };
use crate::systems::{ handle_player_movement, handle_player_fire_bullet, handle_global_input, handle_physics, screen_wrap, despawn_bullets, detect_collisions, AsteroidAsteroidCollision, PlayerAsteroidCollision };
use bevy::prelude::*;
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::systems::asteroid_spawner::spawn_asteroid;

const VIRTUAL_WIDTH: f32 = 1920.0;
const VIRTUAL_HEIGHT: f32 = 1080.0;
const WRAP_BUFFER: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}


struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AsteroidSpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating)
            })
            .add_systems(Startup, (load_textures, create_player, create_camera).chain())
            .add_systems(Update, spawn_asteroid)
            .add_systems(Update, handle_player_movement)
            .add_systems(Update, handle_player_fire_bullet)
            .add_systems(Update, handle_global_input)
            .add_systems(Update, handle_physics)
            .add_systems(Update, screen_wrap)
            .add_systems(Update, despawn_bullets)
            .add_systems(Update, detect_collisions)
            .add_observer(on_asteroid_asteroid_collision)
            .add_observer(on_player_asteroid_collision);
    }
}

fn on_asteroid_asteroid_collision(_ev: On<AsteroidAsteroidCollision>) {

    // TODO: shrink each asteroid by one size, or despawn + spawn explosion if Small
}

fn on_player_asteroid_collision(_ev: On<PlayerAsteroidCollision>) {
    // TODO: despawn player + spawn explosion
}