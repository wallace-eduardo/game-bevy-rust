use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};

use game::{
    atmosphere::AtmospherePlugin,
    camera::CameraPlugin,
    player::*,
    shared::{Game, GameState},
    show_fps::ShowFPSPlugin,
    terrain::*,
};

// fn main() {
//     App::new()

//         .add_systems(Startup, (setup_cameras, setup_game))
//         .add_systems(
//             Update,
//             (
//                 move_player,
//                 /*move_camera, camera_drag,*/ daylight_cycle,
//                 pause,
//             )
//                 .run_if(in_state(GameState::Playing)),
//         )
//         .add_systems(Update, unpause.run_if(in_state(GameState::Paused)))
//         .run();
// }

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            AtmospherePlugin,
            ShowFPSPlugin,
            TerrainPlugin,
            PlayerPlugin,
            CameraPlugin,
            WireframePlugin,
        ))
        .add_systems(Update, toggle_wireframe)
        .init_state::<GameState>()
        .add_systems(Startup, debug_system)
        .run();
}

// fn pause(mut next_state: ResMut<NextState<GameState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         next_state.set(GameState::Paused);
//     }
// }

// fn unpause(
//     mut next_state: ResMut<NextState<GameState>>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         next_state.set(GameState::Playing);
//     }
// }

fn debug_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Simple transform shape just for reference
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.8, 0.8, 0.8)))),
    ));

    // X axis
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.8, 0.0, 0.0)))),
        Transform::from_xyz(1., 0., 0.),
    ));

    // Y axis
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.0, 0.8, 0.0)))),
        Transform::from_xyz(0., 1., 0.),
    ));

    // Z axis
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.0, 0.0, 0.8)))),
        Transform::from_xyz(0., 0., 1.),
    ));
}

fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframes: Query<Entity, (With<WireframeTerrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<WireframeTerrain>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyF) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
        }
        for terrain in &landscapes_wireframes {
            commands.entity(terrain).remove::<Wireframe>();
        }
    }
}
