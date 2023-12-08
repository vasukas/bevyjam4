use bevy::prelude::*;

pub mod barrels;
pub mod enemy;
pub mod particles;
pub mod player;
pub mod terrain;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            terrain::TerrainPlugin,
            enemy::EnemyPlugin,
            barrels::BarrelsPlugin,
            particles::ParticlesPlugin,
        ));
    }
}
