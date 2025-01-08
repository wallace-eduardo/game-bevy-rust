use crate::*;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[derive(Component)]
pub struct Camera;

fn camera_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut settings: ResMut<CameraSettings>,
    mut windows: Query<(&mut Window, Option<&PrimaryWindow>)>,
    mut camera_transforms: Query<&mut Transform, With<Camera>>,
    mut focus: Local<bool>,
) {
    let Some(camera_id) = settings.active_camera else {
        motion.clear();
        return;
    };

    let Ok(mut camera_transform) = camera_transforms.get_mut(camera_id) else {
        error!("Failed to find camera for active camera entity ({camera_id:?})");
        settings.active_camera = None;
        motion.clear();
        return;
    };

    let mut window = match settings.active_window {
        Some(active) => {
            let Ok((window, _)) = windows.get_mut(active) else {
                error!("Failed to find active window ({active:?})");
                settings.active_window = None;
                motion.clear();
                return;
            };

            window
        }
        None => {
            let Some((window, _)) = windows.iter_mut().find(|(_, primary)| primary.is_some())
            else {
                panic!("No primary window found!");
            };

            window
        }
    };

    let mut set_focus = |focused: bool| {
        *focus = focused;
        if !settings.orthographic {
            let grab_mode = match focused {
                true => CursorGrabMode::Confined,
                false => CursorGrabMode::None,
            };
            window.cursor_options.grab_mode = grab_mode;
            window.cursor_options.visible = !focused;
        }
    };

    if keys.just_pressed(KeyCode::Escape) {
        set_focus(false);
    } else if buttons.just_pressed(MouseButton::Left) {
        set_focus(true);
    }

    // When in orthographic mode, mouse capturing is disabled. Movement should therefore always be enabled.
    if *focus || settings.orthographic {
        // rotation
        if !settings.orthographic {
            let mouse_delta = {
                let mut total = Vec2::ZERO;
                for d in motion.read() {
                    total += d.delta;
                }
                total
            };

            let mouse_x = -mouse_delta.x * settings.sensitivity;
            let mouse_y = -mouse_delta.y * settings.sensitivity;

            let mut dof: Vec3 = camera_transform.rotation.to_euler(EulerRot::YXZ).into();

            dof.x += mouse_x;
            // At 90 degrees, yaw gets misinterpreted as roll. Making 89 the limit fixes that.
            dof.y = (dof.y + mouse_y).clamp(-89f32.to_radians(), 89f32.to_radians());
            dof.z = 0f32;

            camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, dof.x, dof.y, dof.z);
        }

        // translation
        {
            let forward = if keys.pressed(KeyCode::KeyW) {
                1f32
            } else {
                0f32
            };

            let backward = if keys.pressed(KeyCode::KeyS) {
                1f32
            } else {
                0f32
            };

            let right = if keys.pressed(KeyCode::KeyD) {
                1f32
            } else {
                0f32
            };

            let left = if keys.pressed(KeyCode::KeyA) {
                1f32
            } else {
                0f32
            };

            let up_cond = if !settings.orthographic {
                keys.pressed(KeyCode::Space)
            } else {
                keys.pressed(KeyCode::KeyW)
            };
            let up = if up_cond { 1f32 } else { 0f32 };

            let down_cond = if !settings.orthographic {
                keys.pressed(KeyCode::ControlLeft)
            } else {
                keys.pressed(KeyCode::KeyS)
            };
            let down = if down_cond { 1f32 } else { 0f32 };

            let speed = if keys.pressed(KeyCode::ShiftLeft) {
                settings.alt_speed
            } else {
                settings.base_speed
            };

            let delta_axial = if settings.orthographic {
                0.0
            } else {
                (forward - backward) * speed
            };
            let delta_lateral = (right - left) * speed;
            let delta_vertical = (up - down) * speed;

            let mut forward = *camera_transform.forward();
            forward.y = 0f32;
            forward = forward.normalize_or_zero(); // fly fast even when look down/up

            let mut right = *camera_transform.right();
            right.y = 0f32; // more of a sanity check
            let up = Vec3::Y;

            let result = forward * delta_axial + right * delta_lateral + up * delta_vertical;

            camera_transform.translation += result * time.delta_secs();
        }
    }

    motion.clear();
}

#[derive(Resource)]
pub struct CameraSettings {
    /// The `Entity` of the active [`Camera`]. (Default: `None`)
    ///
    /// Use this to control which [`Camera`] you are using.
    ///
    /// Setting to `None` will disable the camera mode.
    pub active_camera: Option<Entity>,
    /// The `Entity` of the active `Window`. (Default: `None`)
    ///
    /// Use this to control which `Window` will grab your mouse/hide the cursor.
    ///
    /// If `None`, the primary window will be used.
    pub active_window: Option<Entity>,
    /// The base speed of the active [`Camera`]. (Default: `10.0`)
    ///
    /// Use this to control how fast the [`Camera`] normally moves.
    pub base_speed: f32,
    /// The alternate speed of the active [`Camera`]. (Default: `50.0`)
    ///
    /// Use this to control how fast the [`Camera`] moves when you hold The key to move fast.
    pub alt_speed: f32,
    /// The camera sensitivity of the active [`Camera`]. (Default: `0.001`)
    ///
    /// Use this to control how fast the [`Camera`] turns when you move the mouse.
    pub sensitivity: f32,
    /// Use a control scheme more fit for orthographic (2D) rendering
    ///
    /// Disables mouse capturing and hiding, prevents moving along z-axis and uses W and S for y-axis movement
    pub orthographic: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            active_camera: None,
            active_window: None,
            base_speed: 10.0,
            alt_speed: 50.0,
            sensitivity: 0.001,
            orthographic: false,
        }
    }
}

fn camera_setup(mut commands: Commands) {
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
        Camera,
    ));
}

fn mark_active_camera(cameras: Query<Entity, With<Camera>>, mut settings: ResMut<CameraSettings>) {
    use bevy::ecs::query::QuerySingleError;

    if settings.active_camera.is_none() {
        settings.active_camera = match cameras.get_single() {
            Ok(a) => Some(a),
            Err(QuerySingleError::NoEntities(_)) => {
                warn!("Failed to find a Spectator; Active camera will remain unset.");
                None
            }
            Err(QuerySingleError::MultipleEntities(_)) => {
                warn!("Found more than one Spectator; Active camera will remain unset.");
                None
            }
        };
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_systems(Startup, camera_setup)
            .add_systems(PostStartup, mark_active_camera)
            .add_systems(Update, camera_update);
    }
}
