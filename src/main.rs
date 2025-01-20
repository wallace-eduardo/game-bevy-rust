use bevy::prelude::*;

use game::{
    camera::CameraPlugin, player::*, show_fps::ShowFPSPlugin, terrain::*, BG_COLOR, WH, WW,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            MonitorSelection::Primary,
                        ),
                        resolution: (WW as f32, WH as f32).into(),
                        title: env!("CARGO_PKG_NAME").to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        )))
        .add_plugins((CameraPlugin, ShowFPSPlugin, TerrainPlugin, PlayerPlugin))
        .add_systems(Update, (handle_settings_input, close_on_esc))
        .run();
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn handle_settings_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<ResetTerrainEvent>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    writer.send(ResetTerrainEvent);
}
