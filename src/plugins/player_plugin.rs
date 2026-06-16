use crate::plugins::game_schedule_plugin::{GameSet, StartupSet};
use crate::startup::{create_camera, create_player};
use crate::systems::{despawn_bullets, handle_player_fire_bullet, handle_player_movement};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoScheduleConfigs;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (create_player, create_camera).in_set(StartupSet::Entities),
        )
        .add_systems(
                Update,
                (
                    handle_player_movement,
                    handle_player_fire_bullet,
                )
                    .in_set(GameSet::Input),
            )
        .add_systems(
            Update,
            (
                despawn_bullets,
            )
                .in_set(GameSet::Spawning),
        );
    }
}
