use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::prelude::{Commands, Res};
use crate::resources::world_settings::WorldSettings;

pub fn create_camera(
    world: Res<WorldSettings>,
    mut commands: Commands
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: world.virtual_width,
                height: world.virtual_height,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}