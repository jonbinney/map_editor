mod camera_control;
mod create_world;
mod debug_ui;
mod model;
mod robot_teleop;
mod spawn_robot;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::*;

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn main() {
    App::new()
        // We set the physics gravity to be in the -Z direction because we use the
        // robotics coordinate convention.
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 60.0,
                substeps: 16,
            },
            gravity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -9.81,
            },
            // Start with physics paused for easier debugging.
            physics_pipeline_active: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EguiPlugin)
        // Must be a less hacky way to do this. We need the camera to exist
        // before the camera control starts up, so it can find it and set the
        // initial pose.
        .add_startup_system(create_camera)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(create_world::CreateWorldPlugin)
        .add_plugin(camera_control::CameraControlPlugin)
        .add_plugin(debug_ui::DebugUIPlugin)
        .add_plugin(spawn_robot::SpawnRobotPlugin)
        .add_plugin(robot_teleop::RobotTeleopPlugin)
        .run();
}
