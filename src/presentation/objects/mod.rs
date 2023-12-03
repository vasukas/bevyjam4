use bevy::prelude::*;

mod assets;
mod player;
mod sprite;
mod terrain;
mod utils;

pub use utils::WorldCameraBundle;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            terrain::TerrainPlugin,
            player::PlayerPlugin,
            utils::UtilsPlugin,
            sprite::SpritePlugin,
        ));
    }
}
