use crate::*;

use bevy::{color::palettes::tailwind::*, prelude::*, render::mesh::VertexAttributeValues};
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
pub struct Terrain;

fn terrain_startup(
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
                    let cell = Cell::new(i as f32, j as f32, perlin_noise);
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

    let terrain_height = 70.;
    let noise = BasicMulti::<Perlin>::new(900);

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
            let val = noise.get([pos[0] as f64 / 300., pos[2] as f64 / 300.]);

            pos[1] = val as f32 * terrain_height;
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| {
                let g = *g / terrain_height * 2.;

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
        MeshMaterial3d(materials.add(Color::WHITE)),
        Terrain,
    ));
}
