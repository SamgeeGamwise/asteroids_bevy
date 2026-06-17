use bevy::prelude::{Query, Res, Transform};
use crate::resources::world_settings::WorldSettings;

pub fn screen_wrap(
    world: Res<WorldSettings>,
    mut query: Query<&mut Transform>
) {
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
