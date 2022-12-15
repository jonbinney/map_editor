use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct CameraControlPlugin;
impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraControlState>()
            .add_system(toggle_rotating)
            .add_system(update_camera_pose.after(toggle_rotating));
    }
}

#[derive(Resource)]
pub struct CameraControlState {
    pub rotating: bool,
    pub azimuth: f32,
    pub altitude: f32,
    pub range: f32,
    pub target: Vec3,
}

impl Default for CameraControlState {
    fn default() -> Self {
        Self {
            rotating: false,
            azimuth: 0.0,
            altitude: std::f32::consts::PI / 4.0,
            range: 2.5,
            target: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
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

pub fn update_camera_pose(
    _: Res<Windows>,
    mut state: ResMut<CameraControlState>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_scroll_events: EventReader<MouseWheel>,
) {
    for motion_event in mouse_motion_events.iter() {
        if state.rotating {
            debug!("{:?}", motion_event);

            state.azimuth -= 0.01 * motion_event.delta.x;
            state.altitude -= 0.01 * motion_event.delta.y;
        }
    }
    state.altitude = state
        .altitude
        .clamp(std::f32::EPSILON, std::f32::consts::PI - std::f32::EPSILON);

    for scroll_event in mouse_scroll_events.iter() {
        state.range -= 0.4 * scroll_event.y;
    }
    state.range = state.range.clamp(std::f32::EPSILON, std::f32::INFINITY);

    for (_, mut transform) in camera_query.iter_mut() {
        *transform = compute_camera_transform(state.target, state.altitude, state.azimuth, state.range)
    }
}

pub fn compute_camera_transform(
    target: Vec3,
    altitude: f32,
    azimuth: f32,
    range: f32,
) -> Transform {
    let position = Vec3 {
        x: range * altitude.sin() * azimuth.cos(),
        y: range * altitude.sin() * azimuth.sin(),
        z: range * altitude.cos(),
    };
    Transform {
        translation: position,
        rotation: bevy_look_at(position, target),
        ..default()
    }
}

fn bevy_look_at(position: Vec3, target: Vec3) -> Quat {
    // Camera forward is -z.
    let camera_z = Vec3::normalize(position - target);
    // Camera x is to the right.
    let camera_x = Vec3::Z.cross(camera_z).normalize();
    // Camera y is up.
    let camera_y = camera_z.cross(camera_x);
    Quat::from_mat3(&Mat3::from_cols(camera_x, camera_y, camera_z))
}
