pub mod startup;
pub mod entities;
pub mod systems;

use crate::startup::{ create_camera, create_player, load_textures };
use crate::systems::{ handle_player_movement, handle_player_fire_bullet, handle_global_input, handle_physics, screen_wrap, despawn_bullets };
use bevy::prelude::*;

const VIRTUAL_WIDTH: f32 = 1920.0;
const VIRTUAL_HEIGHT: f32 = 1080.0;
const WRAP_BUFFER: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}


struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (load_textures, create_player, create_camera).chain())
            .add_systems(Update, handle_player_movement)
            .add_systems(Update, handle_player_fire_bullet)
            .add_systems(Update, handle_global_input)
            .add_systems(Update, handle_physics)
            .add_systems(Update, screen_wrap)
            .add_systems(Update, despawn_bullets);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Movement {
    speed: f32,
    turn_speed: f32,
    max_speed: f32,
}

#[derive(Component)]
struct Physics {
    velocity: Vec2,
    acceleration: Vec2,
    drag: f32,
}

#[derive(Component)]
struct Input {
    move_forward: KeyCode,
    move_backward: KeyCode,
    move_left: KeyCode,
    move_right: KeyCode,
    fire: KeyCode,
}

#[derive(Component)]
struct LifeTime {
    timer: Timer
}


#[derive(Resource)]
struct GameTextures {
    player_ship: Handle<Image>,
    bullet: Handle<Image>,
}

