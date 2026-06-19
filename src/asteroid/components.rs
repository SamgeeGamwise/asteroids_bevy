use bevy::prelude::*;

#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

impl Asteroid {
    pub fn new(size: AsteroidSize) -> Self {
        Asteroid { size }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

#[derive(Component)]
pub struct InvulnerabilityTimer {
    pub timer: Timer,
}

impl InvulnerabilityTimer {
    pub fn new(time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Once),
        }
    }
}
