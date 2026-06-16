use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use crate::GameTextures;

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameTextures {
        player_ship: asset_server.load("textures/playerShip1_orange.png"),
        bullet: asset_server.load("textures/laserBlue01.png"),
    });
}

