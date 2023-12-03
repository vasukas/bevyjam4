use bevy::prelude::*;

pub mod game_states;
pub mod level;
pub mod scripts;
pub mod time_master;

pub struct MasterPlugin;

impl Plugin for MasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_states::GameStatesPlugin,
            time_master::TimeMasterPlugin,
            level::LevelPlugin,
            scripts::ScriptsPlugin,
        ));
    }
}
