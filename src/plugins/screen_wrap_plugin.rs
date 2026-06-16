use bevy::app::{App, Plugin, Update};
use crate::systems::screen_wrap;

pub struct ScreenWrapPlugin;

impl Plugin for ScreenWrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, screen_wrap);
    }
}