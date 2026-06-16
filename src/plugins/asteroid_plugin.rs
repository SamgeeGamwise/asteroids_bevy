use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Timer, TimerMode};
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::startup::{create_camera, create_player, load_textures};
use crate::systems::asteroid_spawner::spawn_asteroid;
use crate::systems::{handle_player_fire_bullet, handle_player_movement};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AsteroidSpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating)
            })
            .add_systems(Update, spawn_asteroid);
    }
}