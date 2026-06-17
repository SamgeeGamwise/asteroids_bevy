use bevy::math::Vec2;
use bevy::prelude::{Commands, KeyCode, Res, Sprite, Transform};
use crate::components::{Input, Movement, Physics};
use crate::components::asteroid::AsteroidSize;
use crate::resources::GameTextures;
use crate::components::Player;
use crate::entities::asteroid::create_asteroid;
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::player_settings::PlayerSettings;
use crate::resources::world_settings::WorldSettings;

pub fn create_starting_asteroid(
    asteroid: Res<AsteroidSettings>,
    world: Res<WorldSettings>,
    mut commands: Commands,
    textures: Res<GameTextures>
) {
    create_asteroid(&asteroid, &world, &mut commands, AsteroidSize::Large, textures.large_asteroid.clone(), None);
    create_asteroid(&asteroid, &world, &mut commands, AsteroidSize::Large, textures.large_asteroid.clone(), None);
    create_asteroid(&asteroid, &world, &mut commands, AsteroidSize::Large, textures.large_asteroid.clone(), None);
}