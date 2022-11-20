use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion};
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct CameraControlState {
    pub rotating: bool,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraControlState {
    fn default() -> Self {
        Self {
            rotating: false,
            yaw: 0.0,
            pitch: -std::f32::consts::PI / 2.0,
        }
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
                    ElementState::Pressed => {
                        camera_control_state.rotating = true;
                        window.set_cursor_lock_mode(true);
                    }
                    ElementState::Released => {
                        camera_control_state.rotating = false;
                        window.set_cursor_lock_mode(false);
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
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    for motion_event in mouse_motion_events.iter() {
        for (_, mut transform) in camera_query.iter_mut() {
            if camera_control_state.rotating {
                debug!("{:?}", motion_event);

                camera_control_state.yaw -= 0.01 * motion_event.delta.x;
                camera_control_state.pitch -= 0.01 * motion_event.delta.y;
            }

            camera_control_state.pitch = camera_control_state
                .pitch
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_control_state.yaw)
                * Quat::from_axis_angle(Vec3::X, camera_control_state.pitch);
        }
    }
}
pub struct CameraControlPlugin;
impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraControlState>()
            .add_system(toggle_rotating)
            .add_system(rotate_camera);
    }
}
