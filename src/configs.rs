// Window
pub const WW: f32 = 1200.0;
pub const WH: f32 = 900.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_W: usize = 16;
pub const TILE_H: usize = 16;
pub const SPRITE_SHEET_W: usize = 8;
pub const SPRITE_SHEET_H: usize = 8;

// World
pub const NUM_WORLD_DECORATIONS: usize = 500;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2500.0;

// Player
pub const PLAYER_SPEED: f32 = 2.0;
pub const PLAYER_HEALTH: f32 = 100.0;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 20000;
pub const ENEMY_DAMAGE: f32 = 1.0;
pub const SPAWN_RATE_PER_SECOND: usize = 500;
pub const ENEMY_HEALTH: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_TIME_SECS: f32 = 0.5;
pub const BULLET_SPEED: f32 = 15.0;
pub const BULLET_DAMAGE: f32 = 15.0;

pub const NUM_BULLETS_PER_SHOT: usize = 10;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
