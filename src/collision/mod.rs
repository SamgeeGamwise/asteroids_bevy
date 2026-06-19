mod events;
mod handlers;

pub use events::detect_collisions;

use self::handlers::{
    on_asteroid_asteroid_collision, on_bullet_asteroid_collision, on_player_asteroid_collision,
};
use crate::schedule::GameSet;
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions.in_set(GameSet::Collision))
            .add_observer(on_asteroid_asteroid_collision)
            .add_observer(on_player_asteroid_collision)
            .add_observer(on_bullet_asteroid_collision);
    }
}
