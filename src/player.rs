use crate::*;
use bevy::{math::*, prelude::*, utils::Instant};

pub struct PlayerPlugin;

#[derive(Component)]
struct WalkTrail(Instant);
#[derive(Component)]
struct Player;
#[derive(Resource)]
struct PlayerSpriteIndex(usize);
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Resource, Default)]
struct PlayerDirection(f32);
#[derive(Resource)]
struct WalkTrailTimer(Timer);
#[derive(Resource)]
struct DefaultAtlasHandle(pub Option<Handle<TextureAtlasLayout>>);
#[derive(Resource, Default)]
pub struct CurrentPlayerChunkPosition(pub (i32, i32));
#[derive(Event)]
pub struct PlayerChunkUpdateEvent(pub (i32, i32));
#[derive(Resource)]
struct DefaultSpriteSheet(pub Option<Handle<Image>>);

pub const PLAYER_SPEED: f32 = 2.0;
pub const PLAYER_FISH_SPEED: f32 = 1.5;
pub const PLAYER_ANIMATION_INTERVAL: f32 = 0.3;
pub const WALK_TRAIL_TIMER: f32 = 1.2;
pub const TRAIL_LIFE_SPAN: f32 = 5.0;
pub const PLAYER_JUMP_TIME: f32 = 0.3;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerState {
    #[default]
    Idle,
    Walk,
    Jump(Instant),
    Swim,
}

impl PlayerState {
    fn on_land(&self) -> bool {
        match self {
            PlayerState::Idle => true,
            PlayerState::Walk => true,
            _ => false,
        }
    }

    fn walking(&self) -> bool {
        *self == PlayerState::Walk
    }

    fn swimming(&self) -> bool {
        *self == PlayerState::Swim
    }

    fn jumping(&self) -> bool {
        match self {
            PlayerState::Jump(_) => true,
            _ => false,
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .insert_resource(PlayerSpriteIndex(0))
            .insert_resource(PlayerDirection(0.0))
            .insert_resource(CurrentPlayerChunkPosition::default())
            .insert_resource(WalkTrailTimer(Timer::from_seconds(
                WALK_TRAIL_TIMER,
                TimerMode::Repeating,
            )))
            .insert_resource(DefaultAtlasHandle(None))
            .insert_resource(DefaultSpriteSheet(None))
            .add_event::<PlayerChunkUpdateEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_player_state)
            .add_systems(Update, camera_follow_player)
            .add_systems(Update, handle_player_input)
            .add_systems(Update, spawn_walk_trail)
            .add_systems(Update, update_player_chunk_pos)
            .add_systems(Update, clean_old_walk_trails)
            .add_systems(Update, update_player_sprite);
    }
}

fn setup(
    mut commands: Commands,
    mut handle: ResMut<DefaultAtlasHandle>,
    mut sheet: ResMut<DefaultSpriteSheet>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load(SPRITE_SHEET_PATH);
    sheet.0 = Some(texture_handle);
    let texture_atlas = TextureAtlasLayout::from_grid(
        uvec2(TILE_W as u32, TILE_H as u32),
        SPRITE_SHEET_W as u32,
        SPRITE_SHEET_H as u32,
        Some(UVec2::splat(SPRITE_PADDING)),
        Some(UVec2::splat(SPRITE_SHEET_OFFSET)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    handle.0 = Some(texture_atlas_handle);

    commands.spawn((
        Sprite::from_atlas_image(
            sheet.0.clone().unwrap(),
            TextureAtlas {
                layout: handle.0.clone().unwrap(),
                index: PLAYER_SPRITE_INDEX,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
            .with_translation(vec3(0.0, 0.0, 2.0)),
        Player,
        AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_INTERVAL,
            TimerMode::Repeating,
        )),
    ));
}

fn update_player_state(
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut sprite_index: ResMut<PlayerSpriteIndex>,
    ground_tiles: Res<GroundTiles>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let transform = player_query.single_mut();
    let (x, y) = (transform.translation.x, transform.translation.y);
    let (x, y) = world_to_grid(x, y);
    let (x, y) = center_to_top_left_grid(x, y);
    let is_ground = ground_tiles.0.contains(&(x as i32, y as i32));

    if !is_ground && player_state.on_land() {
        next_player_state.set(PlayerState::Jump(Instant::now()));
    }
    if is_ground && player_state.swimming() {
        next_player_state.set(PlayerState::Jump(Instant::now()));
    }

    let transform = player_query.single_mut();
    let (x, y) = (transform.translation.x, transform.translation.y);
    let (x, y) = world_to_grid(x, y);
    let (x, y) = center_to_top_left_grid(x, y);
    let is_ground = ground_tiles.0.contains(&(x as i32, y as i32));

    match player_state.get() {
        PlayerState::Jump(jumped_at) => {
            if jumped_at.elapsed().as_secs_f32() > PLAYER_JUMP_TIME {
                next_player_state.set(if is_ground {
                    PlayerState::Idle
                } else {
                    PlayerState::Swim
                });
                sprite_index.0 = 0;
            }
        }
        _ => {}
    }
}

fn update_player_sprite(
    time: Res<Time>,
    mut sprite_index: ResMut<PlayerSpriteIndex>,
    player_state: Res<State<PlayerState>>,
    mut query: Query<(&mut Sprite, &mut AnimationTimer), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, mut timer) = query.single_mut();
    timer.tick(time.delta());

    if player_state.walking() && timer.finished() {
        sprite_index.0 = (sprite_index.0 + 1) % 3;
    }
    if player_state.jumping() && timer.finished() {
        sprite_index.0 = (sprite_index.0 + 1) % 3;
    }

    sprite.texture_atlas.as_mut().unwrap().index = if player_state.on_land() {
        sprite_index.0 + PLAYER_SPRITE_INDEX
    } else if player_state.jumping() {
        sprite_index.0 + PLAYER_SPRITE_INDEX + 3
    } else {
        49
    };
}

fn update_player_chunk_pos(
    mut chunk_position: ResMut<CurrentPlayerChunkPosition>,
    mut chunk_update_event: EventWriter<PlayerChunkUpdateEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let transform = player_query.single();
    let (x, y) = (transform.translation.x, transform.translation.y);
    let (a, b) = world_to_grid(x, y);
    let (a, b) = center_to_top_left_grid(a, b);
    let (x, y) = grid_to_chunk(a, b);

    let (old_x, old_y) = chunk_position.0;
    if old_x == x && old_y == y {
        return;
    }

    chunk_update_event.send(PlayerChunkUpdateEvent((x, y)));
    chunk_position.0 = (x, y);
}

fn handle_player_input(
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_direction: ResMut<PlayerDirection>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }
    if player_state.jumping() {
        return;
    }

    let mut transform = player_query.single_mut();
    let w_key = keys.pressed(KeyCode::KeyW);
    let a_key = keys.pressed(KeyCode::KeyA);
    let s_key = keys.pressed(KeyCode::KeyS);
    let d_key = keys.pressed(KeyCode::KeyD);
    let speed_scale = if keys.pressed(KeyCode::ShiftLeft) {
        5.0
    } else {
        1.0
    };
    let mut direction = Vec3::ZERO;

    if w_key {
        direction.y += 1.0;
    }
    if s_key {
        direction.y -= 1.0;
    }
    if a_key {
        direction.x -= 1.0;
    }
    if d_key {
        direction.x += 1.0;
    }

    if w_key || s_key || a_key || d_key {
        let player_angle = direction.y.atan2(direction.x);
        let sprite_angle = if player_state.on_land() {
            0.0
        } else {
            player_angle
        };
        let speed = if player_state.on_land() {
            PLAYER_SPEED
        } else {
            PLAYER_FISH_SPEED
        };
        let new_pos = transform.translation + direction.normalize() * speed * speed_scale;

        if !new_pos.is_nan() {
            transform.translation = new_pos;
        }

        transform.rotation = Quat::from_rotation_z(sprite_angle);
        player_direction.0 = player_angle;
        next_player_state.set(if player_state.on_land() {
            PlayerState::Walk
        } else {
            PlayerState::Swim
        });
    } else {
        next_player_state.set(if player_state.on_land() {
            PlayerState::Idle
        } else {
            PlayerState::Swim
        });
    }
}

fn spawn_walk_trail(
    time: Res<Time>,
    mut commands: Commands,
    player_state: Res<State<PlayerState>>,
    player_angle: Res<PlayerDirection>,
    image_handle: Res<DefaultAtlasHandle>,
    sheet: ResMut<DefaultSpriteSheet>,
    mut timer: ResMut<WalkTrailTimer>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());
    if player_query.is_empty() {
        return;
    }

    if !timer.0.finished() || !player_state.walking() {
        return;
    }

    let transform = player_query.single_mut();
    commands.spawn((
        Sprite::from_atlas_image(
            sheet.0.clone().unwrap(),
            TextureAtlas {
                layout: image_handle.0.clone().unwrap(),
                index: 50usize,
            },
        ),
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32 - 1.0))
            .with_translation(vec3(transform.translation.x, transform.translation.y, 1.0))
            .with_rotation(Quat::from_rotation_z(player_angle.0)),
        WalkTrail(Instant::now()),
    ));
}

fn clean_old_walk_trails(
    mut commands: Commands,
    query: Query<(Entity, &WalkTrail), With<WalkTrail>>,
) {
    if query.is_empty() {
        return;
    }

    for (entity, trail) in query.iter() {
        if trail.0.elapsed().as_secs_f32() > TRAIL_LIFE_SPAN {
            commands.entity(entity).despawn();
        }
    }
}

fn camera_follow_player(
    mut camera_query: Query<(&Camera, &mut Transform), Without<Player>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (_, mut camera_transform) = camera_query.get_single_mut().unwrap();
    let player_transform = player_query.get_single_mut().unwrap();

    camera_transform.translation = camera_transform.translation.lerp(
        vec3(
            player_transform.translation.x,
            player_transform.translation.y,
            0.0,
        ),
        0.05,
    );
}
