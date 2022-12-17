mod camera_control;
mod create_world;
mod spawn_robot;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn main() {
    App::new()
        // We set the physics gravity to be in the -Z direction because we use the
        // robotics coordinate convention.
        .insert_resource(RapierConfiguration {
            gravity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -9.81,
            },
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // Must be a less hacky way to do this. We need the camera to exist
        // before the camera control starts up, so it can find it and set the
        // initial pose.
        .add_startup_system(create_camera)
        .add_plugin(create_world::CreateWorldPlugin)
        .add_plugin(camera_control::CameraControlPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(spawn_robot::SpawnRobotPlugin)
        .run();
}
