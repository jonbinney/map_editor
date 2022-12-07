use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::f32::consts::FRAC_PI_2;
pub struct CameraControlPlugin;
impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraControlState>()
            .add_system(toggle_rotating)
            .add_system(rotate_camera.after(toggle_rotating))
            .add_system(update_camera_pose.after(rotate_camera));
    }
}

#[derive(Resource)]
pub struct CameraControlState {
    pub rotating: bool,
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraControlState {
    fn default() -> Self {
        Self {
            rotating: false,
            position: Vec3 {
                x: -2.0,
                y: 0.0,
                z: 3.0,
            },
            yaw: 0.0,
            pitch: 1.1,
        }
    }
}

pub fn update_camera_pose(
    camera_control_state: ResMut<CameraControlState>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in camera_query.iter_mut() {
        transform.translation = camera_control_state.position;
        transform.rotation = Quat::from_rotation_z(camera_control_state.yaw)
            * Quat::from_rotation_y(camera_control_state.pitch)
            * Quat::from_mat3(&Mat3::from_cols(-Vec3::Y, Vec3::Z, -Vec3::X));
    }
}

pub fn toggle_rotating(
    mut windows: ResMut<Windows>,
    mut camera_control_state: ResMut<CameraControlState>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
) {
    for event in mouse_button_events.iter() {
        match event.button {
            MouseButton::Right => {
                let window = windows.get_primary_mut().unwrap();
                match event.state {
                    ButtonState::Pressed => {
                        camera_control_state.rotating = true;
                        window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Confined);
                    }
                    ButtonState::Released => {
                        camera_control_state.rotating = false;
                        window.set_cursor_grab_mode(bevy::window::CursorGrabMode::None);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn rotate_camera(
    _: Res<Windows>,
    mut camera_control_state: ResMut<CameraControlState>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    for motion_event in mouse_motion_events.iter() {
        if camera_control_state.rotating {
            debug!("{:?}", motion_event);

            camera_control_state.yaw -= 0.01 * motion_event.delta.x;
            camera_control_state.pitch -= 0.01 * motion_event.delta.y;
        }

        camera_control_state.pitch = camera_control_state.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
    }
}
