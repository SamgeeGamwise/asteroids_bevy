use bevy::app::{App, Update};
use bevy::prelude::{IntoScheduleConfigs, On, Plugin};
use crate::plugins::game_schedule_plugin::GameSet;
use crate::systems::{detect_collisions, AsteroidAsteroidCollision, PlayerAsteroidCollision};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, detect_collisions.in_set(GameSet::Collision))
            .add_observer(on_asteroid_asteroid_collision)
            .add_observer(on_player_asteroid_collision);
    }
}


fn on_asteroid_asteroid_collision(_ev: On<AsteroidAsteroidCollision>) {

    // TODO: shrink each asteroid by one size, or despawn + spawn explosion if Small
}

fn on_player_asteroid_collision(_ev: On<PlayerAsteroidCollision>) {
    // TODO: despawn player + spawn explosion
}