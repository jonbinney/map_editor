mod camera_control;
mod create_world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Must be a less hacky way to do this. We need the camera to exist
        // before the camera control starts up, so it can find it and set the
        // initial pose.
        .add_startup_system(create_camera)
        .add_plugin(create_world::CreateWorldPlugin)
        .add_plugin(camera_control::CameraControlPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
