use bevy::prelude::{Commands, Entity, On, Query, Res, Transform};
use crate::components::asteroid::{Asteroid, AsteroidSize};
use crate::components::InvulnerabilityTimer;
use crate::entities::asteroid::create_asteroid;
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::GameTextures;
use crate::resources::world_settings::WorldSettings;
use crate::systems::{AsteroidAsteroidCollision, PlayerAsteroidCollision};

pub fn on_asteroid_asteroid_collision(
    _ev: On<AsteroidAsteroidCollision>,
    query: Query<(Entity, &Transform, &Asteroid, &InvulnerabilityTimer)>,
    mut commands: Commands,
    asteroid_settings: Res<AsteroidSettings>,
    world_settings: Res<WorldSettings>,
    game_textures: Res<GameTextures>,
) {
    let Ok((entity_a, transform_a, asteroid_a, invulnerability_a)) = query.get(_ev.entity_a) else {
        return;
    };

    let Ok((entity_b,transform_b, asteroid_b, invulnerability_b)) = query.get(_ev.entity_b) else {
        return;
    };

    if invulnerability_a.timer.is_finished() && invulnerability_b.timer.is_finished() {
        break_asteroid(entity_a, asteroid_a, transform_a, &mut commands, &asteroid_settings, &world_settings, &game_textures);
        break_asteroid(entity_b, asteroid_b, transform_b, &mut commands, &asteroid_settings, &world_settings, &game_textures);
    }
}

fn break_asteroid(
    entity: Entity,
    asteroid: &Asteroid,
    transform: &Transform,
    commands: &mut Commands,
    asteroid_settings: &AsteroidSettings,
    world_settings: &WorldSettings,
    game_textures: &GameTextures,
) {

    match asteroid.size {
        AsteroidSize::Large => {
            create_asteroid(&asteroid_settings, &world_settings, commands, AsteroidSize::Medium, game_textures.medium_asteroid.clone(), Some(transform.translation.clone()));
            create_asteroid(&asteroid_settings, &world_settings, commands, AsteroidSize::Medium, game_textures.medium_asteroid.clone(), Some(transform.translation.clone()));
        },
        AsteroidSize::Medium => {
            create_asteroid(&asteroid_settings, &world_settings, commands, AsteroidSize::Small, game_textures.small_asteroid.clone(), Some(transform.translation.clone()));
            create_asteroid(&asteroid_settings, &world_settings, commands, AsteroidSize::Small, game_textures.small_asteroid.clone(), Some(transform.translation.clone()));
            create_asteroid(&asteroid_settings, &world_settings, commands, AsteroidSize::Small, game_textures.small_asteroid.clone(), Some(transform.translation.clone()));
        },
        AsteroidSize::Small => {

        }
    }

    commands.entity(entity).despawn();
}

pub fn on_player_asteroid_collision(_ev: On<PlayerAsteroidCollision>) {
    // TODO: despawn player + spawn explosion
}