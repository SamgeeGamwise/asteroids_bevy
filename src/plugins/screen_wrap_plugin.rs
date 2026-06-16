use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoScheduleConfigs;
use crate::plugins::game_schedule_plugin::GameSet;
use crate::systems::screen_wrap;

pub struct ScreenWrapPlugin;

impl Plugin for ScreenWrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, screen_wrap.in_set(GameSet::Wrap));
    }
}