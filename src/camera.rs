use crate::*;
use bevy::prelude::*;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

fn camera_setup(mut commands: Commands) {
    let camera_initial_position: Transform = Transform::from_xyz(
        BOARD_SIZE_COLS as f32 / 2.0,
        1.0,
        BOARD_SIZE_ROWS as f32 / 2.0,
    )
    .with_rotation(Quat::from_rotation_y(-PI / 2.0));

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
            target_focus: camera_initial_position,
            // Change starting zoom level
            target_zoom: 0.5,
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
            ..default()
        },
        Msaa::Sample4,
        bevy_atmosphere::plugin::AtmosphereCamera::default(), // Marks camera as having a skybox, by default it doesn't specify the render layers the skybox can be seen on
    ));

    // Help text
    commands.spawn(Text::new(format!(
        "Press WASD to move the camera\n\
                     Press E or Q to rotate the camera\n\
                     Rotate the camera with right click\n\
                     Drag pan with middle click\n\
                     Arrow keys to move the player\n\
                     Press X to toggle wireframe\n\
                     Camera Initial position: (x: {:.2}, y:{:.2}, z:{:.2})",
        camera_initial_position.translation.x,
        camera_initial_position.translation.y,
        camera_initial_position.translation.z
    )));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, camera_setup);
        //.add_systems(Update, camera_update);
    }
}
