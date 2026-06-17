use bevy::app::{App, Update};
use bevy::prelude::{IntoScheduleConfigs, Plugin};
use crate::events::collision::{on_asteroid_asteroid_collision, on_bullet_asteroid_collision, on_player_asteroid_collision};
use crate::plugins::game_schedule_plugin::GameSet;
use crate::systems::{detect_collisions};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, detect_collisions.in_set(GameSet::Collision))
            .add_observer(on_asteroid_asteroid_collision)
            .add_observer(on_player_asteroid_collision)
            .add_observer(on_bullet_asteroid_collision);
    }
}
