use crate::plugins::game_schedule_plugin::StartupSet;
use crate::startup::load_textures;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoScheduleConfigs;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_textures.in_set(StartupSet::Assets));
    }
}
