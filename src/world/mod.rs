mod systems;

use crate::schedule::{GameSet, StartupSet};
use bevy::prelude::*;

pub use systems::{create_camera, screen_wrap};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera.in_set(StartupSet::World))
            .add_systems(Update, screen_wrap.in_set(GameSet::Wrap));
    }
}
