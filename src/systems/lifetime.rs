use bevy::prelude::{Commands, Entity, Query, Res, Time};
use crate::components::{ LifeTime };

pub fn despawn_bullets(mut commands: Commands, mut query: Query<(Entity, &mut LifeTime)>, time: Res<Time>) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
