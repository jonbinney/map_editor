use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rapier3d::dynamics::JointAxis;

use crate::model;

pub struct RobotTeleopPlugin;
impl Plugin for RobotTeleopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(robot_teleop);
    }
}

pub fn robot_teleop(keys: Res<Input<KeyCode>>, mut query: Query<(&model::LinkName, &mut MultibodyJoint)>) {
    let max_wheel_velocity = 40.0;
    let mut left_wheel_velocity = 0.0;
    let mut right_wheel_velocity = 0.0;
    if keys.pressed(KeyCode::A) {
        println!("A pressed");
        left_wheel_velocity = max_wheel_velocity;
    }
    if keys.pressed(KeyCode::D) {
        println!("D pressed");
        right_wheel_velocity = max_wheel_velocity;
    }

    for (link_name, mut joint) in query.iter_mut() {
        let velocity = match &link_name.0[..] {
            "right_wheel" => right_wheel_velocity,
            "left_wheel" => left_wheel_velocity,
            _ => 0.0
        };
        joint.data.set_motor_velocity(JointAxis::AngX, velocity, 1.0);
        println!("Set velocity for {} to {}", link_name.0, velocity)
    }
}
