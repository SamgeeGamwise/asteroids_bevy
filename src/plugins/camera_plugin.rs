use crate::plugins::game_schedule_plugin::{ StartupSet};
use crate::startup::{create_camera};
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoScheduleConfigs;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera.in_set(StartupSet::World));
    }
}
