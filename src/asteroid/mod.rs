pub mod components;
pub mod spawn;
pub mod systems;

use self::systems::{create_starting_asteroid, spawn_asteroid, tick_invulnerability};
use crate::schedule::{GameSet, StartupSet};
use crate::settings::AsteroidSettings;
use bevy::prelude::*;

pub use components::{Asteroid, AsteroidSize, InvulnerabilityTimer};

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl AsteroidSpawnTimer {
    pub fn new(time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
        }
    }
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        let spawn_timer = app.world().resource::<AsteroidSettings>().spawn_timer;

        app.insert_resource(AsteroidSpawnTimer::new(spawn_timer))
            .add_systems(
                Startup,
                create_starting_asteroid.in_set(StartupSet::Entities),
            )
            .add_systems(Update, tick_invulnerability.in_set(GameSet::Collision))
            .add_systems(Update, spawn_asteroid.in_set(GameSet::Spawning));
    }
}
