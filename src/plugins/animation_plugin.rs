use bevy::app::{App, Plugin, Update};
use crate::systems::animate_sprite::animate_sprites;
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}