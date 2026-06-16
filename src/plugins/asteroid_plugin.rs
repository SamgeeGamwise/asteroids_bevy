use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Timer, TimerMode};
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::systems::asteroid_spawner::spawn_asteroid;

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