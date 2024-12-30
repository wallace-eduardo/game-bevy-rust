use bevy::prelude::*;
use rand::rngs::StdRng;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    //Paused,
}

pub struct Cell {
    pub height: f32,
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub row: usize,
    pub col: usize,
    pub move_cooldown: Timer,
}

#[derive(Resource, Default)]
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub player: Player,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Random(pub StdRng);
