use crate::assets::GameTextures;
use crate::asteroid::AsteroidSpawnTimer;
use crate::asteroid::components::{AsteroidSize, InvulnerabilityTimer};
use crate::asteroid::spawn::create_asteroid;
use crate::settings::{AsteroidSettings, WorldSettings};
use bevy::prelude::*;

pub fn create_starting_asteroid(
    asteroid: Res<AsteroidSettings>,
    world: Res<WorldSettings>,
    mut commands: Commands,
    textures: Res<GameTextures>,
) {
    create_asteroid(
        &asteroid,
        &world,
        &mut commands,
        AsteroidSize::Large,
        textures.large_asteroid.clone(),
        None,
    );
    create_asteroid(
        &asteroid,
        &world,
        &mut commands,
        AsteroidSize::Large,
        textures.large_asteroid.clone(),
        None,
    );
    create_asteroid(
        &asteroid,
        &world,
        &mut commands,
        AsteroidSize::Large,
        textures.large_asteroid.clone(),
        None,
    );
}

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
        create_asteroid(
            &asteroid,
            &world,
            &mut commands,
            AsteroidSize::Large,
            textures.large_asteroid.clone(),
            None,
        );
    }
}

pub fn tick_invulnerability(
    mut query: Query<(Entity, &mut InvulnerabilityTimer)>,
    time: Res<Time>,
) {
    for (_entity, mut invulnerability) in &mut query {
        invulnerability.timer.tick(time.delta());
    }
}
