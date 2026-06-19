use crate::schedule::StartupSet;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTextures {
    pub player_ship: Handle<Image>,
    pub bullet: Handle<Image>,
    pub small_asteroid: Handle<Image>,
    pub medium_asteroid: Handle<Image>,
    pub large_asteroid: Handle<Image>,
    pub explosion: Vec<Handle<Image>>,
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameTextures {
        player_ship: asset_server.load("textures/playerShip1_orange.png"),
        bullet: asset_server.load("textures/laserBlue01.png"),
        small_asteroid: asset_server.load("textures/meteor/meteorGrey_small.png"),
        medium_asteroid: asset_server.load("textures/meteor/meteorGrey_med.png"),
        large_asteroid: asset_server.load("textures/meteor/meteorGrey_big.png"),
        explosion: vec![
            asset_server.load("textures/explosion/001.png"),
            asset_server.load("textures/explosion/002.png"),
            asset_server.load("textures/explosion/003.png"),
            asset_server.load("textures/explosion/004.png"),
            asset_server.load("textures/explosion/005.png"),
            asset_server.load("textures/explosion/006.png"),
            asset_server.load("textures/explosion/007.png"),
            asset_server.load("textures/explosion/008.png"),
            asset_server.load("textures/explosion/009.png"),
        ],
    });
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_textures.in_set(StartupSet::Assets));
    }
}
