pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;
pub const WIDTH: u32 = WINDOW_WIDTH * 3 / 4;
pub const HEIGHT: u32 = WINDOW_HEIGHT;
pub const FWIDTH: f64 = WIDTH as f64;
pub const FHEIGHT: f64 = HEIGHT as f64;
pub const SCREEN_RECT: [f64; 4] = [0., 0., FWIDTH, FHEIGHT];

pub const PLAYER_SPEED: f64 = 2.;
pub const PLAYER_SIZE: f64 = 16.;
pub const PLAYER_INVINCIBLE_TIME: u32 = 128;
pub const PLAYER_LIVES: u32 = 3;
pub const ENEMY_SIZE: f64 = 8.;
pub const BOSS_SIZE: f64 = 16.;
pub const CENTIPEDE_SIZE: f64 = 16.;
pub const BULLET_SIZE: f64 = 8.;
pub const LONG_BULLET_SIZE: [f64; 2] = [8., 4.];
pub const BULLET_SPEED: f64 = 5.;
pub const MISSILE_SPEED: f64 = 3.;
pub const LIGHT_WIDTH: f64 = 3.;
pub const EXPLODE_SIZE: f64 = 8.;
pub const EXPLODE2_SIZE: f64 = 16.;
pub const ITEM_SIZE: f64 = 6.;
pub const ITEM2_SIZE: f64 = 12.;

pub const LIGHTNING_ACCEL: f64 = 8.0;
pub const LIGHTNING_FEEDBACK: f64 = 0.1;
pub const LIGHTNING_VERTICES: u32 = 32;
