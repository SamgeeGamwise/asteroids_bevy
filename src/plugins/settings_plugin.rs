use bevy::app::{App, Plugin};
use crate::resources::asteroid_settings::AsteroidSettings;
use crate::resources::bullet_settings::BulletSettings;
use crate::resources::player_settings::PlayerSettings;
use crate::resources::world_settings::WorldSettings;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WorldSettings {
                virtual_width: 1920.0,
                virtual_height: 1080.0,
                wrap_buffer: 64.0,
            })
            .insert_resource(PlayerSettings {
                thrust: 1200.0,
                turn_speed: 2.0,
                max_speed: 500.0,
                drag: 1.0,
                hitbox_radius: 24.0,
            })
            .insert_resource(BulletSettings {
                speed: 800.0,
                lifetime_seconds: 1.0,
                spawn_offset: 60.0,
            })
            .insert_resource(AsteroidSettings {
                spawn_timer: 8.0,
                invulnerability_timer: 1.0,
                scale: 2.0,
                speed: 500.0,
                max_speed: 500.0,
                angular_speed: 1.0,
                hitbox_radius_large: 75.0,
                hitbox_radius_medium: 35.0,
                hitbox_radius_small: 20.0,
            });
    }
}