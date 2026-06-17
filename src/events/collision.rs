use bevy::prelude::{Commands, Entity, On, Query, Res, Transform};
use bevy::sprite::Sprite;
use crate::components::asteroid::{Asteroid, AsteroidSize};
use crate::components::{Bullet, InvulnerabilityTimer, Player};
use crate::components::explosion::Explosion;
use crate::components::sprite_animation::SpriteAnimation;
use crate::entities::asteroid::create_asteroid;
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::explosion_settings::ExplosionSettings;
use crate::resources::GameTextures;
use crate::resources::world_settings::WorldSettings;
use crate::systems::{AsteroidAsteroidCollision, PlayerAsteroidCollision};
use crate::systems::collision::BulletAsteroidCollision;

pub fn on_asteroid_asteroid_collision(
    _ev: On<AsteroidAsteroidCollision>,
    query: Query<(Entity, &Transform, &Asteroid, &InvulnerabilityTimer)>,
    mut commands: Commands,
    explosion: Res<ExplosionSettings>,
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
        break_asteroid(entity_a, asteroid_a, transform_a, &mut commands, &explosion, &asteroid_settings, &world_settings, &game_textures);
        break_asteroid(entity_b, asteroid_b, transform_b, &mut commands, &explosion, &asteroid_settings, &world_settings, &game_textures);
    }
}

fn break_asteroid(
    entity: Entity,
    asteroid: &Asteroid,
    transform: &Transform,
    commands: &mut Commands,
    explosion: &ExplosionSettings,
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

    commands.spawn((
        Explosion,
        Sprite::from_image(game_textures.explosion[0].clone()),
        SpriteAnimation::new(explosion.frame_per_time, game_textures.explosion.clone()),
        Transform::from_translation(transform.translation.clone()),
    ));
    commands.entity(entity).despawn();
}

pub fn on_player_asteroid_collision(
    _ev: On<PlayerAsteroidCollision>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid, &InvulnerabilityTimer)>,
    player_query: Query<(Entity, &Transform, &Player)>,
    mut commands: Commands,
    explosion: Res<ExplosionSettings>,
    asteroid_settings: Res<AsteroidSettings>,
    world_settings: Res<WorldSettings>,
    game_textures: Res<GameTextures>,
) {
    let Ok((player_entity, player_transform, player)) = player_query.get(_ev.player) else {
        return;
    };

    let Ok((asteroid_entity, asteroid_transform, asteroid, invulnerability)) = asteroid_query.get(_ev.asteroid) else {
        return;
    };

    if invulnerability.timer.is_finished() {
        break_asteroid(asteroid_entity, asteroid, asteroid_transform, &mut commands,  &explosion, &asteroid_settings, &world_settings, &game_textures);

        commands.spawn((
            Explosion,
            Sprite::from_image(game_textures.explosion[0].clone()),
            SpriteAnimation::new(explosion.frame_per_time, game_textures.explosion.clone()),
            Transform::from_translation(player_transform.translation.clone()),
        ));
        commands.entity(player_entity).despawn();
    }
}

pub fn on_bullet_asteroid_collision(
    _ev: On<BulletAsteroidCollision>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid)>,
    bullet_query: Query<Entity>,
    mut commands: Commands,
    explosion: Res<ExplosionSettings>,
    asteroid_settings: Res<AsteroidSettings>,
    world_settings: Res<WorldSettings>,
    game_textures: Res<GameTextures>,
) {
    let Ok((bullet_entity)) = bullet_query.get(_ev.bullet) else {
        return;
    };

    let Ok((asteroid_entity, asteroid_transform, asteroid)) = asteroid_query.get(_ev.asteroid) else {
        return;
    };

    break_asteroid(asteroid_entity, asteroid, asteroid_transform, &mut commands,  &explosion, &asteroid_settings, &world_settings, &game_textures);

    commands.spawn((
        Explosion,
        Sprite::from_image(game_textures.explosion[0].clone()),
        SpriteAnimation::new(explosion.frame_per_time, game_textures.explosion.clone()),
        Transform::from_translation(asteroid_transform.translation.clone()),
    ));
    commands.entity(bullet_entity).despawn();
}