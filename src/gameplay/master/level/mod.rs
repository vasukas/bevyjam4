use bevy::prelude::*;

pub mod current;
pub mod data;
pub mod database;
pub mod spawn;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            current::CurrentPlugin,
            database::DatabasePlugin,
            spawn::SpawnPlugin,
        ));
    }
}
