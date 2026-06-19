use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::*;

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub frames: Vec<Handle<Image>>,
    pub current_frame: usize,
}

impl SpriteAnimation {
    pub fn new(time: f32, textures: Vec<Handle<Image>>) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
            frames: textures,
            current_frame: 0,
        }
    }
}

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

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}
