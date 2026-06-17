use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use crate::resources::GameTextures;

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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

