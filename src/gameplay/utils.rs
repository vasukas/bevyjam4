use super::master::level::data::HALF_TILE;
use super::master::level::data::TILE_SIZE;
use bevy::prelude::*;

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
