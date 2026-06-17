use bevy::prelude::{Commands, Entity, Query, Res, Sprite, Time};
use crate::components::sprite_animation::SpriteAnimation;

pub fn animate_sprites(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut SpriteAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut animation) in &mut query {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.current_frame += 1;

            if animation.current_frame >= animation.frames.len() {
                commands.entity(entity).despawn();
                continue;
            }

            sprite.image = animation.frames[animation.current_frame].clone();
        }
    }
}