use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoScheduleConfigs;
use crate::plugins::game_schedule_plugin::GameSet;
use crate::systems::handle_physics;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_physics.in_set(GameSet::Physics));
    }
}