use bevy::prelude::*;

mod game_script;
pub mod game_states;
pub mod level;
pub mod script_points;
pub mod time_master;

pub struct MasterPlugin;

impl Plugin for MasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_states::GameStatesPlugin,
            time_master::TimeMasterPlugin,
            level::LevelPlugin,
            script_points::ScriptsPlugin,
            game_script::GameScriptPlugin,
        ));
    }
}
