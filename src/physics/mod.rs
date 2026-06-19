use crate::schedule::GameSet;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Physics {
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub acceleration: Vec2,
    pub drag: f32,
    pub max_speed: f32,
}

pub fn handle_physics(mut query: Query<(&mut Physics, &mut Transform)>, time: Res<Time>) {
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

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_physics.in_set(GameSet::Physics));
    }
}
