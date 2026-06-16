use bevy::prelude::{Query, Res, Time, Transform};
use crate::components::Physics;

pub fn handle_physics(
    mut query: Query<(&mut Physics, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut physics, mut transform) in &mut query {
        let dt = time.delta_secs();

        let acceleration = physics.acceleration;
        physics.velocity += acceleration * dt;

        let drag = (-physics.drag * dt).exp();
        physics.velocity *= drag;

        if physics.velocity.length() > physics.max_speed {
            physics.velocity = physics.velocity.normalize() * physics.max_speed;
        }
        
        transform.rotate_z(physics.angular_velocity * dt);

        let velocity = physics.velocity;
        transform.translation += (velocity * dt).extend(0.0);
    }
}


