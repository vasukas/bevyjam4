use super::master::level::data::HALF_TILE;
use super::master::level::data::TILE_SIZE;
use bevy::prelude::*;
use leafwing_input_manager::orientation::Orientation;
use leafwing_input_manager::orientation::Rotation;
use std::f32::consts::TAU;

/// Tile which position belongs to
pub fn pos_to_tile(mut pos: Vec2) -> IVec2 {
    if pos.x < 0. {
        pos.x -= 1.
    }
    if pos.y < 0. {
        pos.y -= 1.
    }

    let tile = (pos / TILE_SIZE).floor().as_ivec2();
    tile
}

/// Center of the tile
pub fn tile_center(tile: IVec2) -> Vec2 {
    tile.as_vec2() * TILE_SIZE + HALF_TILE
}

/// Center of tile which position belongs to
pub fn pos_to_tile_center(pos: Vec2) -> Vec2 {
    tile_center(pos_to_tile(pos))
}

//

/// Gradually rotates entity in target direction
#[derive(Component)]
pub struct RotateToTarget {
    pub target_dir: Vec2,
    pub rotation_speed: f32,
}

impl RotateToTarget {
    pub fn new(rotation_speed: f32) -> Self {
        Self {
            target_dir: Vec2::Y,
            rotation_speed,
        }
    }

    pub fn new_from_time(seconds_full_360: f32) -> Self {
        Self::new(TAU / seconds_full_360)
    }
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_to_target);
    }
}

pub fn rotate_to_target(mut entities: Query<(&mut Transform, &RotateToTarget)>, time: Res<Time>) {
    for (mut transform, target) in entities.iter_mut() {
        let target_angle = target.target_dir.angle_between(Vec2::X);
        let target_rot = Quat::from_rotation_z(target_angle);

        let max_delta = target.rotation_speed * time.delta_seconds();
        let max_rotation = Rotation::from_radians(max_delta);

        transform
            .rotation
            .rotate_towards(target_rot, Some(max_rotation));
    }
}
