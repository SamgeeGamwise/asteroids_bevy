use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoScheduleConfigs;
use crate::entities::asteroid::create_asteroid;
use crate::plugins::game_schedule_plugin::{GameSet, StartupSet};
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::startup::asteroids::create_starting_asteroid;
use crate::systems::asteroid_spawner::spawn_asteroid;
use crate::systems::invulnerability::tick_invulnerabilty;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        let asteroid_settings = app.world().resource::<AsteroidSettings>();

        app
            .insert_resource(AsteroidSpawnTimer::new(asteroid_settings.spawn_timer))
            .add_systems(Startup, create_starting_asteroid.in_set(StartupSet::Entities))
            .add_systems(Update, tick_invulnerabilty.in_set(GameSet::Collision))
            .add_systems(Update, spawn_asteroid.in_set(GameSet::Spawning));
    }
}