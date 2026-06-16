use bevy::prelude::{Query, Res, Time, Transform};
use crate::{Movement, Physics};

pub fn handle_physics(
    mut query: Query<(&mut Physics, &mut Transform, &Movement)>,
    time: Res<Time>,
) {
    for (mut physics, mut transform, movement) in &mut query {
        let dt = time.delta_secs();

        let acceleration = physics.acceleration;
        physics.velocity += acceleration * dt;

        let drag = (-physics.drag * dt).exp();
        physics.velocity *= drag;

        if physics.velocity.length() > movement.max_speed {
            physics.velocity = physics.velocity.normalize() * movement.max_speed;
        }

        let velocity = physics.velocity;
        transform.translation += (velocity * dt).extend(0.0);
    }
}


