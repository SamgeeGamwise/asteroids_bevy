use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameTextures {
    pub player_ship: Handle<Image>,
    pub bullet: Handle<Image>,
    pub small_asteroid: Handle<Image>,
    pub medium_asteroid: Handle<Image>,
    pub large_asteroid: Handle<Image>,
    pub explosion: Vec<Handle<Image>>,
}

