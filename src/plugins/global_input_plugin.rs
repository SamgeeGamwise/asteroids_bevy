use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoScheduleConfigs;
use crate::plugins::game_schedule_plugin::GameSet;
use crate::systems::handle_global_input;

pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_global_input.in_set(GameSet::Input));
    }
}