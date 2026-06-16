use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, Timer, TimerMode};
use crate::plugins::game_schedule_plugin::GameSet;
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::systems::asteroid_spawner::spawn_asteroid;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AsteroidSpawnTimer::new(2.0))
            .add_systems(Update, spawn_asteroid.in_set(GameSet::Spawning));
    }
}