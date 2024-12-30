use crate::*;
use bevy::prelude::*;
use shared::{Game, Player};

impl Player {
    fn default() -> Self {
        Player {
            entity: None,
            row: BOARD_SIZE_ROWS / 2,
            col: BOARD_SIZE_COLS / 2,
            move_cooldown: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands, mut game: ResMut<Game>, asset_server: Res<AssetServer>) {
    // Initial game state
    game.player = Player::default();

    // spawn the game character
    game.player.entity = Some(
        commands
            .spawn((
                Transform {
                    translation: Vec3::new(
                        game.player.row as f32,
                        game.board[game.player.col][game.player.row].height,
                        game.player.col as f32,
                    ),
                    rotation: Quat::from_rotation_y(-PI / 2.),
                    ..default()
                },
                SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("alien.glb"))),
            ))
            .id(),
    );
}

// control the game character
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if game.player.row < BOARD_SIZE_ROWS - 1 {
                game.player.row += 1;
            }
            rotation = -PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if game.player.row > 0 {
                game.player.row -= 1;
            }
            rotation = PI / 2.;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if game.player.col < BOARD_SIZE_COLS - 1 {
                game.player.col += 1;
            }
            rotation = PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if game.player.col > 0 {
                game.player.col -= 1;
            }
            rotation = 0.0;
            moved = true;
        }

        // move on the board
        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.player.row as f32,
                    game.board[game.player.col][game.player.row].height,
                    game.player.col as f32,
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..default()
            };
        }
    }
}
