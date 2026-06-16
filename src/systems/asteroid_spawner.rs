use bevy::prelude::{Commands, Res, ResMut, Time};
use crate::components::asteroid::AsteroidSize;
use crate::entities::asteroid::create_asteroid;
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::resources::GameTextures;

pub fn spawn_asteroid(
    mut commands: Commands,
    mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
    textures: Res<GameTextures>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());

    if asteroid_spawn_timer.timer.just_finished() {
        create_asteroid(&mut commands, AsteroidSize::Large, textures.large_asteroid.clone());
    }
}