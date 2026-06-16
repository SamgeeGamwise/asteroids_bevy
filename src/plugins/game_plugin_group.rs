use bevy::app::{PluginGroup, PluginGroupBuilder};
use super::global_input_plugin::GlobalInputPlugin;
use super::physics_plugin::PhysicsPlugin;
use super::screen_wrap_plugin::ScreenWrapPlugin;
use super::asset_plugin::AssetPlugin;
use super::asteroid_plugin::AsteroidPlugin;
use super::collision_plugin::CollisionPlugin;
use super::player_plugin::PlayerPlugin;

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetPlugin)
            .add(PlayerPlugin)
            .add(AsteroidPlugin)
            .add(CollisionPlugin)
            .add(PhysicsPlugin)
            .add(GlobalInputPlugin)
            .add(ScreenWrapPlugin)
    }
}