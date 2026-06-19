use crate::settings::WorldSettings;
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::prelude::*;

pub fn create_camera(world: Res<WorldSettings>, mut commands: Commands) {
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

pub fn screen_wrap(world: Res<WorldSettings>, mut query: Query<&mut Transform>) {
    let half_width = world.virtual_width / 2.0;
    let half_height = world.virtual_height / 2.0;

    for mut transform in &mut query {
        if transform.translation.x > half_width + world.wrap_buffer {
            transform.translation.x = -half_width - world.wrap_buffer;
        } else if transform.translation.x < -half_width - world.wrap_buffer {
            transform.translation.x = half_width + world.wrap_buffer;
        }

        if transform.translation.y > half_height + world.wrap_buffer {
            transform.translation.y = -half_height - world.wrap_buffer;
        } else if transform.translation.y < -half_height - world.wrap_buffer {
            transform.translation.y = half_height + world.wrap_buffer;
        }
    }
}
