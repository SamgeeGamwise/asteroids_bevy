use bevy::prelude::*;
use crate::components::asteroid::{Asteroid, AsteroidSize};
use crate::components::Player;

const ASTEROID_BASE_RADIUS: f32 = 50.0;
const PLAYER_RADIUS: f32 = 24.0;

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

fn asteroid_radius(size: AsteroidSize) -> f32 {
    let scale = match size {
        AsteroidSize::Large => 2.0,
        AsteroidSize::Medium => 1.0,
        AsteroidSize::Small => 0.75,
    };
    ASTEROID_BASE_RADIUS * scale
}

pub fn detect_collisions(
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
            if dist < asteroid_radius(asteroid_a.size) + asteroid_radius(asteroid_b.size) {
                commands.trigger(AsteroidAsteroidCollision { entity_a, entity_b });
            }
        }
    }

    if let Ok((player_entity, player_transform)) = player_query.single() {
        for (asteroid_entity, asteroid_transform, asteroid) in &asteroids {
            let dist = player_transform.translation.distance(asteroid_transform.translation);
            if dist < PLAYER_RADIUS + asteroid_radius(asteroid.size) {
                commands.trigger(PlayerAsteroidCollision {
                    player: player_entity,
                    asteroid: *asteroid_entity,
                });
            }
        }
    }
}
