use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Timer, TimerMode};
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::startup::{create_camera, create_player, load_textures};
use crate::systems::asteroid_spawner::spawn_asteroid;
use crate::systems::{handle_global_input, handle_physics, handle_player_fire_bullet, handle_player_movement, screen_wrap};

pub struct ScreenWrapPlugin;

impl Plugin for ScreenWrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, screen_wrap);
    }
}