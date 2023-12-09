use bevy::prelude::*;

mod animation_ctl;
mod assets;
mod barrels;
mod enemy;
mod other_objects;
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
            sprite::SpritePlugin,
            animation_ctl::AnimationCtlPlugin,
            utils::UtilsPlugin,
            enemy::EnemyPlugin,
            other_objects::OtherObjectsPlugin,
            barrels::BarrelsPlugin,
        ));
    }
}
