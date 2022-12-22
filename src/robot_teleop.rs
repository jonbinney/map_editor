use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::model;

pub struct RobotTeleopPlugin;
impl Plugin for RobotTeleopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(robot_teleop);
    }
}

pub fn robot_teleop(keys: Res<Input<KeyCode>>, mut query: Query<(&model::LinkName, &mut ImpulseJoint)>) {
    let max_wheel_velocity = 4.0;
    let mut left_wheel_velocity = 0.0;
    let mut right_wheel_velocity = 0.0;
    if keys.pressed(KeyCode::A) {
        left_wheel_velocity = max_wheel_velocity;
    }
    if keys.pressed(KeyCode::D) {
        right_wheel_velocity = max_wheel_velocity;
    }

    for (link_name, mut joint) in query.iter_mut() {
        let velocity = match &link_name.0[..] {
            "right_wheel" => right_wheel_velocity,
            "left_wheel" => left_wheel_velocity,
            _ => 0.0
        };
        if let Some(revolute_joint) = joint.data.as_revolute_mut() {
            revolute_joint.set_motor(0.0, velocity, 0.0, 10.0);
        }
    }
}
