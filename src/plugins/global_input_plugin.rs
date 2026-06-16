use bevy::app::{App, Plugin, Update};
use crate::systems::handle_global_input;

pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_global_input);
    }
}