use bevy::{app::*, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        // Msaa::Off,
        Camera2d,
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.01,
            max_scale: 7.5,
            ..default()
        },
        // OrthographicProjection {
        //     near: -10000.0,
        //     far: 1000000.0,
        //     ..OrthographicProjection::default_3d()
        // },
    ));
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(Startup, camera_setup);
    }
}
