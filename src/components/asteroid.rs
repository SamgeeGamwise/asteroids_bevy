use bevy::prelude::Component;

#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize
}

impl Asteroid {
    pub fn new(size: AsteroidSize) -> Self {
        Asteroid { size }
    }
}

#[derive(Clone, Copy)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}