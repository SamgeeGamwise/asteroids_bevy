use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::prelude::Commands;
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};

pub fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: VIRTUAL_WIDTH,
                height: VIRTUAL_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}