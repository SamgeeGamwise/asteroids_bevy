use bevy::app::{App, Plugin, Startup, Update};
use crate::startup::{create_camera, create_player, load_textures};
use crate::systems::asteroid_spawner::spawn_asteroid;
use crate::systems::{despawn_bullets, handle_player_fire_bullet, handle_player_movement};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (create_player, create_camera))
            .add_systems(Update, handle_player_movement)
            .add_systems(Update, handle_player_fire_bullet)
            .add_systems(Update, despawn_bullets);
    }
}