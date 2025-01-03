use crate::*;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{rngs::StdRng, Rng, SeedableRng};
use shared::{Cell, Game, Random};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, terrain_startup);
    }
}

fn terrain_startup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let mut rng = if std::env::var("GITHUB_ACTIONS") == Ok("true".to_string()) {
        // Make the game play out the same way every time, this is useful for testing purposes.
        StdRng::seed_from_u64(19878367467713)
    } else {
        StdRng::from_entropy()
    };

    let perlin = Perlin::new(rng.gen());

    // spawn the game board
    let cell_scene =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("Nature (Kenney)/ground_grass.glb"));
    game.board = (0..BOARD_SIZE_COLS)
        .map(|j| {
            (0..BOARD_SIZE_ROWS)
                .map(|i| {
                    let perlin_noise = perlin
                        .get([i as f64 * PERLIN_NOISE_SCALE, j as f64 * PERLIN_NOISE_SCALE])
                        as f32;
                    //let random_height_jitter = rng.gen_range(-0.1..0.1);
                    let cell = Cell::new(i as f32, perlin_noise, j as f32);
                    commands.spawn((
                        Transform::from_translation(cell.position),
                        SceneRoot(cell_scene.clone()),
                    ));
                    cell
                })
                .collect()
        })
        .collect();

    commands.insert_resource(Random(rng));
}
