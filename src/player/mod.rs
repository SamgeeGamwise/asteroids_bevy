pub mod components;
pub mod systems;

use self::systems::{
    create_player, handle_global_input, handle_player_fire_bullet, handle_player_movement,
};
use crate::bullet::systems::despawn_bullets;
use crate::schedule::{GameSet, StartupSet};
use bevy::prelude::*;

pub use components::{Input, Movement, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player.in_set(StartupSet::Entities))
            .add_systems(
                Update,
                (
                    handle_global_input,
                    handle_player_movement,
                    handle_player_fire_bullet,
                )
                    .in_set(GameSet::Input),
            )
            .add_systems(Update, despawn_bullets.in_set(GameSet::Spawning));
    }
}
