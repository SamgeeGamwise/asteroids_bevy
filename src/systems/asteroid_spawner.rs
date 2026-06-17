use bevy::prelude::{Commands, Res, ResMut, Time, World};
use crate::components::asteroid::AsteroidSize;
use crate::entities::asteroid::create_asteroid;
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::asteroid_spawn_timer::AsteroidSpawnTimer;
use crate::resources::GameTextures;
use crate::resources::world_settings::WorldSettings;

pub fn spawn_asteroid(
    world: Res<WorldSettings>,
    asteroid: Res<AsteroidSettings>,
    mut commands: Commands,
    mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
    textures: Res<GameTextures>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());

    if asteroid_spawn_timer.timer.just_finished() {
        create_asteroid(&asteroid, &world, &mut commands, AsteroidSize::Large, textures.large_asteroid.clone(), None);
    }
}