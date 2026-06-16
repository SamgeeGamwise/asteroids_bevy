use bevy::prelude::{Query, Transform};
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, WRAP_BUFFER};

pub fn screen_wrap(mut query: Query<&mut Transform>) {
    let half_width = VIRTUAL_WIDTH / 2.0;
    let half_height = VIRTUAL_HEIGHT / 2.0;

    for mut transform in &mut query {
        if transform.translation.x > half_width + WRAP_BUFFER {
            transform.translation.x = -half_width - WRAP_BUFFER;
        } else if transform.translation.x < -half_width - WRAP_BUFFER {
            transform.translation.x = half_width + WRAP_BUFFER;
        }

        if transform.translation.y > half_height + WRAP_BUFFER {
            transform.translation.y = -half_height - WRAP_BUFFER;
        } else if transform.translation.y < -half_height - WRAP_BUFFER {
            transform.translation.y = half_height + WRAP_BUFFER;
        }
    }
}
