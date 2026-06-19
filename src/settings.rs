use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldSettings {
    pub virtual_width: f32,
    pub virtual_height: f32,
    pub wrap_buffer: f32,
}

#[derive(Resource)]
pub struct PlayerSettings {
    pub thrust: f32,
    pub turn_speed: f32,
    pub max_speed: f32,
    pub drag: f32,
    pub hitbox_radius: f32,
}

#[derive(Resource)]
pub struct BulletSettings {
    pub speed: f32,
    pub lifetime_seconds: f32,
    pub spawn_offset: f32,
    pub hitbox_radius: f32,
}

#[derive(Resource)]
pub struct AsteroidSettings {
    pub spawn_timer: f32,
    pub invulnerability_timer: f32,
    pub scale: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub angular_speed: f32,
    pub hitbox_radius_large: f32,
    pub hitbox_radius_medium: f32,
    pub hitbox_radius_small: f32,
}

#[derive(Resource)]
pub struct ExplosionSettings {
    pub frame_per_time: f32,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings {
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
            hitbox_radius: 5.0,
        })
        .insert_resource(ExplosionSettings {
            frame_per_time: 0.1,
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
