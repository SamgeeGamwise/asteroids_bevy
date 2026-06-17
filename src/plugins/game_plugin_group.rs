use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::plugins::animation_plugin::AnimationPlugin;
use crate::plugins::camera_plugin::CameraPlugin;
use crate::plugins::game_schedule_plugin::GameSchedulePlugin;
use crate::plugins::settings_plugin::SettingsPlugin;
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
            .add(GameSchedulePlugin)
            .add(SettingsPlugin)
            .add(AssetPlugin)
            .add(PlayerPlugin)
            .add(CameraPlugin)
            .add(AsteroidPlugin)
            .add(CollisionPlugin)
            .add(PhysicsPlugin)
            .add(GlobalInputPlugin)
            .add(ScreenWrapPlugin)
            .add(AnimationPlugin)
    }
}