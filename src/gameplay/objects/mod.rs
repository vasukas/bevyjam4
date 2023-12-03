use bevy::prelude::*;

pub mod player;
pub mod terrain;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, terrain::TerrainPlugin));
    }
}
