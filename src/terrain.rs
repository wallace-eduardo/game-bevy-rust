use crate::*;

use bevy::{color::palettes::tailwind::*, prelude::*, render::mesh::VertexAttributeValues};
use bevy_rts_camera::Ground;
use noise::{BasicMulti, NoiseFn, Perlin};
use rand::{rngs::StdRng, Rng, SeedableRng};
use shared::{Cell, Game, Random};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, terrain_startup);
    }
}

#[derive(Component)]
pub struct WireframeTerrain;

pub fn terrain_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = if std::env::var("GITHUB_ACTIONS") == Ok("true".to_string()) {
        // Make the game play out the same way every time, this is useful for testing purposes.
        StdRng::seed_from_u64(19878367467713)
    } else {
        StdRng::from_entropy()
    };

    let perlin = BasicMulti::<Perlin>::new(rng.gen());

    ////////////////////////////////////////////////// grid terrain///////////////////////////////////////////////////
    // spawn the game board
    let cell_scene =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("Nature (Kenney)/ground_grass.glb"));
    game.board = (0..BOARD_SIZE_ROWS)
        .map(|i| {
            (0..BOARD_SIZE_COLS)
                .map(|j| {
                    let perlin_noise = perlin
                        .get([i as f64 * PERLIN_NOISE_SCALE, j as f64 * PERLIN_NOISE_SCALE])
                        as f32;
                    let random_height_jitter = rng.gen_range(-0.1..0.1);
                    let cell = Cell::new(i as f32, random_height_jitter, j as f32, perlin_noise);
                    commands.spawn((
                        Transform::from_translation(cell.position),
                        SceneRoot(cell_scene.clone()),
                        Ground,
                    ));
                    cell
                })
                .collect()
        })
        .collect();
    ////////////////////////////////////////////// mesh terrain/////////////////////////////////////////////////////////
    commands.insert_resource(Random(rng));

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000.0, 1000.0)
            .subdivisions(200),
    );

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            let val = perlin.get([
                pos[0] as f64 * PERLIN_NOISE_SCALE,
                pos[2] as f64 * PERLIN_NOISE_SCALE,
            ]);

            pos[1] = val as f32 * TERRAIN_HEIGHT;
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| {
                let g = *g / TERRAIN_HEIGHT * 2.;

                if g > 0.8 {
                    (Color::LinearRgba(LinearRgba {
                        red: 20.,
                        green: 20.,
                        blue: 20.,
                        alpha: 1.,
                    }))
                    .to_linear()
                    .to_f32_array()
                } else if g > 0.3 {
                    Color::from(AMBER_800).to_linear().to_f32_array()
                } else if g < -0.8 {
                    Color::BLACK.to_linear().to_f32_array()
                } else {
                    (Color::from(GREEN_400).to_linear()).to_f32_array()
                }
            })
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    terrain.compute_normals();
    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        // MeshMaterial3d(materials.add(Color::WHITE)),
        MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 0., // Change alpha to make visible
        }))),
        WireframeTerrain,
        Ground,
    ));
}
