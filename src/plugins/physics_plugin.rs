use bevy::app::{App, Plugin, Update};
use crate::systems::handle_physics;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_physics);
    }
}