mod assets;
mod asteroid;
mod bullet;
mod collision;
mod fx;
mod physics;
mod player;
mod schedule;
mod settings;
mod world;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(schedule::GameSchedulePlugin)
            .add(settings::SettingsPlugin)
            .add(assets::AssetPlugin)
            .add(player::PlayerPlugin)
            .add(world::WorldPlugin)
            .add(asteroid::AsteroidPlugin)
            .add(collision::CollisionPlugin)
            .add(physics::PhysicsPlugin)
            .add(fx::AnimationPlugin)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePluginGroup)
        .run();
}
