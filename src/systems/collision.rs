use bevy::prelude::*;
use crate::components::asteroid::{Asteroid, AsteroidSize};
use crate::components::Player;
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::player_settings::PlayerSettings;

#[derive(Event)]
pub struct AsteroidAsteroidCollision {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Event)]
pub struct PlayerAsteroidCollision {
    pub player: Entity,
    pub asteroid: Entity,
}

fn asteroid_radius(asteroid: &AsteroidSettings, size: AsteroidSize) -> f32 {
    match size {
        AsteroidSize::Large => asteroid.hitbox_radius_large,
        AsteroidSize::Medium => asteroid.hitbox_radius_medium,
        AsteroidSize::Small => asteroid.hitbox_radius_small,
    }
}

pub fn detect_collisions(
    asteroid_settings: Res<AsteroidSettings>,
    player: Res<PlayerSettings>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,
) {
    let asteroids: Vec<(Entity, &Transform, &Asteroid)> = asteroid_query.iter().collect();

    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let (entity_a, transform_a, asteroid_a) = asteroids[i];
            let (entity_b, transform_b, asteroid_b) = asteroids[j];
            let dist = transform_a.translation.distance(transform_b.translation);
            if dist < asteroid_radius(&asteroid_settings, asteroid_a.size) + asteroid_radius(&asteroid_settings, asteroid_b.size) {
                commands.trigger(AsteroidAsteroidCollision { entity_a, entity_b });
            }
        }
    }

    if let Ok((player_entity, player_transform)) = player_query.single() {
        for (asteroid_entity, asteroid_transform, asteroid) in &asteroids {
            let dist = player_transform.translation.distance(asteroid_transform.translation);
            if dist < player.hitbox_radius + asteroid_radius(&asteroid_settings, asteroid.size) {
                commands.trigger(PlayerAsteroidCollision {
                    player: player_entity,
                    asteroid: *asteroid_entity,
                });
            }
        }
    }
}
