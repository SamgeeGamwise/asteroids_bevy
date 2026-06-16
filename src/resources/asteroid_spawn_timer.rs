use bevy::prelude::*;

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl AsteroidSpawnTimer {
    pub fn new(time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating)
        }
    }
}