pub const DAY_NIGHT_CYCLE: u64 = 24 * 60; // 24 minutes in seconds
pub const BOARD_SIZE_ROWS: usize = 14;
pub const BOARD_SIZE_COLS: usize = 21;
pub const CAMERA_INITIAL_FOCUS: [f32; 3] = [
    BOARD_SIZE_ROWS as f32 / 2.0,
    0.0,
    BOARD_SIZE_COLS as f32 / 2.0 - 0.5,
];
pub use std::f32::consts::PI;
pub const PERLIN_NOISE_SCALE: f64 = 0.1;
