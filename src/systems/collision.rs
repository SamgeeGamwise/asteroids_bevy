use bevy::prelude::*;
use crate::components::asteroid::{Asteroid, AsteroidSize};
use crate::components::{Bullet, Player};
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::bullet_settings::BulletSettings;
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

#[derive(Event)]
pub struct BulletAsteroidCollision {
    pub bullet: Entity,
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
    bullet_settings: Res<BulletSettings>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut commands: Commands,
) {
    let asteroids: Vec<(Entity, &Transform, &Asteroid)> = asteroid_query.iter().collect();
    let bullets: Vec<(Entity, &Transform, &Bullet)> = bullet_query.iter().collect();

    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let (entity_a, transform_a, asteroid_a) = asteroids[i];
            let (entity_b, transform_b, asteroid_b) = asteroids[j];
            let dist = transform_a.translation.distance(transform_b.translation);
            if dist < asteroid_radius(&asteroid_settings, asteroid_a.size) + asteroid_radius(&asteroid_settings, asteroid_b.size) {
                commands.trigger(AsteroidAsteroidCollision { entity_a, entity_b });
            }
        }

        for j in 0..bullets.len() {
            let (asteroid_entity, asteroid_transform, asteroid) = asteroids[i];
            let (bullet_entity, bullet_transform, _bullet) = bullets[j];
            let dist = asteroid_transform.translation.distance(bullet_transform.translation);

            if dist < bullet_settings.hitbox_radius + asteroid_radius(&asteroid_settings, asteroid.size) {
                commands.trigger(BulletAsteroidCollision { bullet: bullet_entity, asteroid: asteroid_entity });
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
