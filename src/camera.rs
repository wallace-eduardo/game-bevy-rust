use crate::*;
use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

#[derive(Resource)]
struct InitialCameraPosition(Transform);

impl Default for InitialCameraPosition {
    fn default() -> Self {
        Self(
            Transform::from_xyz(
                BOARD_SIZE_COLS as f32 / 2.0,
                1.0,
                BOARD_SIZE_ROWS as f32 / 2.0,
            )
            .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
        )
    }
}
fn camera_setup(mut commands: Commands) {
    let initial_transform = InitialCameraPosition::default();

    // Spawn our camera
    commands.spawn((
        RtsCamera {
            // Increase max height (decrease min zoom)
            height_max: 50.0,
            // Increase min height (decrease max zoom)
            height_min: 5.0,
            // Change the angle of the camera to 35 degrees
            min_angle: 35.0f32.to_radians(),
            // Decrease smoothing
            smoothness: 0.1,
            // Starting position
            target_focus: initial_transform.0,
            // Change starting zoom level
            target_zoom: 0.5,
            // Increasing bounds, can move 200.0 in any direction starting at world center
            bounds: Aabb2d::new(Vec2::ZERO, Vec2::new(200.0, 200.0)),
            // Disable dynamic angle (angle of camera will stay at `min_angle`)
            // dynamic_angle: false,
            ..default()
        },
        RtsCameraControls {
            // Change pan controls to WASD
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            // Rotate the camera with right click
            button_rotate: MouseButton::Right,
            // Keep the mouse cursor in place when rotating
            lock_on_rotate: true,
            // Drag pan with middle click
            button_drag: Some(MouseButton::Middle),
            // Keep the mouse cursor in place when dragging
            lock_on_drag: true,
            // Change the width of the area that triggers edge pan. 0.1 is 10% of the window height.
            edge_pan_width: 0.1,
            // Increase pan speed
            pan_speed: 25.0,
            // Reduce rotate speed
            key_rotate_speed: 3.0,
            ..default()
        },
        Msaa::Sample4,
        bevy_atmosphere::plugin::AtmosphereCamera::default(), // Marks camera as having a skybox, by default it doesn't specify the render layers the skybox can be seen on
    ));

    commands.insert_resource(initial_transform);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, camera_setup)
            .add_systems(Update, camera_reset);
    }
}

fn camera_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut RtsCamera>,
    initial_position: Res<InitialCameraPosition>,
) {
    if keyboard_input.pressed(KeyCode::KeyR) {
        if let Ok(mut camera) = query.get_single_mut() {
            camera.target_focus = initial_position.0;
        }
    }
}
