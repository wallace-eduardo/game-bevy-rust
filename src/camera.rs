use crate::*;
use bevy::prelude::*;
use bevy_spectator::{Spectator, SpectatorPlugin};

// fn camera_drag(
//     mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
//     mut mouse_motion: EventReader<MouseMotion>,
//     mouse_input: Res<ButtonInput<MouseButton>>,
//     mut camera: Query<&mut Transform, With<Camera3d>>,
// ) {
//     if mouse_input.pressed(MouseButton::Middle) {
//         if let Ok(mut window) = primary_window.get_single_mut() {
//             const SENSITIVITY: f32 = 0.00012;
//             match window.cursor_options.grab_mode {
//                 CursorGrabMode::None => {
//                     window.cursor_options.grab_mode = CursorGrabMode::Confined;
//                     window.cursor_options.visible = false;
//                     for mut transform in camera.iter_mut() {
//                         for ev in mouse_motion.read() {
//                             let (mut yaw, mut pitch, _) =
//                                 transform.rotation.to_euler(EulerRot::YXZ);
//                             match window.cursor_options.grab_mode {
//                                 CursorGrabMode::None => (),
//                                 _ => {
//                                     // Using smallest of height or width ensures equal vertical and horizontal sensitivity
//                                     let window_scale = window.height().min(window.width());
//                                     pitch -= (SENSITIVITY * ev.delta.y * window_scale).to_radians();
//                                     yaw -= (SENSITIVITY * ev.delta.x * window_scale).to_radians();
//                                 }
//                             }

//                             pitch = pitch.clamp(-1.54, 1.54);

//                             // Order is important to prevent unintended roll
//                             transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw)
//                                 * Quat::from_axis_angle(Vec3::X, pitch);
//                         }
//                     }
//                 }
//                 _ => {
//                     window.cursor_options.grab_mode = CursorGrabMode::None;
//                     window.cursor_options.visible = true;
//                 }
//             }
//         } else {
//             warn!("Primary window not found for `cursor_grab`!");
//         }
//     }
// }

// fn move_camera(
//     time: Res<Time>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut mouse_wheel: EventReader<MouseWheel>,
//     mut camera: Query<&mut Transform, With<Camera3d>>,
// ) {
//     let movement_speed = 30.0;
//     let rotation_speed = 3.5;
//     let scroll_speed = 60.0;
//     let mut direction = Vec3::ZERO;
//     let mut rotation = 0.0;

//     // Get movement input
//     if keyboard_input.pressed(KeyCode::KeyW) {
//         direction.z -= movement_speed;
//     }
//     if keyboard_input.pressed(KeyCode::KeyS) {
//         direction.z += movement_speed;
//     }
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         direction.x -= movement_speed;
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         direction.x += movement_speed;
//     }

//     if keyboard_input.pressed(KeyCode::KeyR) {
//         // reset camera position and focus
//         for mut transform in camera.iter_mut() {
//             transform.translation = Vec3::new(
//                 -(BOARD_SIZE_ROWS as f32 / 2.0),
//                 2.0 * BOARD_SIZE_COLS as f32 / 3.0,
//                 BOARD_SIZE_COLS as f32 / 2.0 - 0.5,
//             );
//             transform.look_at(Vec3::from(RESET_FOCUS), Vec3::Y);
//         }
//     }

//     // Get scroll input
//     for ev in mouse_wheel.read() {
//         if ev.y > 0.0 {
//             direction.y -= scroll_speed; // Scroll up
//         } else if ev.y < 0.0 {
//             direction.y += scroll_speed; // Scroll down
//         }
//     }

//     // Get rotation input
//     if keyboard_input.pressed(KeyCode::KeyQ) {
//         rotation += rotation_speed;
//     }
//     if keyboard_input.pressed(KeyCode::KeyE) {
//         rotation -= rotation_speed;
//     }

//     // Apply movement and rotation
//     for mut transform in camera.iter_mut() {
//         if direction != Vec3::ZERO {
//             // Convert world direction to object space
//             let local_direction = transform.rotation * direction;
//             transform.translation += local_direction * time.delta_secs();
//         }

//         if rotation != 0.0 {
//             transform.rotate_y(rotation * time.delta_secs());
//         }
//     }
// }

// // // update the score displayed during the game
// // fn scoreboard_system(game: Res<Game>, mut display: Single<&mut Text>) {
// //     display.0 = format!("Money: {}", game.player.money);
// // }

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpectatorPlugin)
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(
    //         -(BOARD_SIZE_ROWS as f32 / 2.0),
    //         2.0 * BOARD_SIZE_COLS as f32 / 3.0,
    //         BOARD_SIZE_COLS as f32 / 2.0 - 0.5,
    //     )
    //     .looking_at(Vec3::from(RESET_FOCUS), Vec3::Y),
    //     AtmosphereCamera::default(),
    // ));

    // Spawn our camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            -(BOARD_SIZE_ROWS as f32 / 2.0),
            2.0 * BOARD_SIZE_COLS as f32 / 3.0,
            BOARD_SIZE_COLS as f32 / 2.0 - 0.5,
        )
        .looking_at(Vec3::from(CAMERA_INITIAL_FOCUS), Vec3::Y),
        Msaa::Sample4,
        bevy_atmosphere::plugin::AtmosphereCamera::default(), // Marks camera as having a skybox, by default it doesn't specify the render layers the skybox can be seen on
        Spectator, // Marks camera as spectator (specific to bevy_spectator)
    ));
}
