use super::master::level::data::TILE_SIZE;
use std::time::Duration;

pub const OVERLOAD_ENEMY_REGULAR: f32 = 5.;
pub const OVERLOAD_RADIUS: f32 = TILE_SIZE * 5.;
pub const OVERLOAD_BOSS: f32 = 8.;

pub const OVERLOAD_PROJECTILE_IMPACT: f32 = 0.03;
pub const OVERLOAD_FIRE_IMPACT: f32 = 0.05;
pub const OVERLOAD_BURNING_BARREL: f32 = 0.25;
pub const OVERLOAD_EXPLOSION: f32 = 1.;
pub const OVERLOAD_OVERLOADED: f32 = 0.4;

pub const OVERLOAD_DURATION_PARTICLE: Duration = Duration::from_millis(1000);
pub const OVERLOAD_DURATION_EXPLOSION: Duration = Duration::from_millis(5000);
pub const OVERLOAD_DURATION_OVERLOADED: Duration = Duration::from_millis(3000);

pub const DURATION_FIREBALL: Duration = Duration::from_millis(600);
pub const SPEED_FIREBALL_PLAYER: f32 = 5.;
pub const SPEED_FIREBALL_EXPLOSION: f32 = 6.;
