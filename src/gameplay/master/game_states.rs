use super::level::current::LevelCommand;
use crate::utils::misc_utils::ExtendedEventReader;
use bevy::prelude::*;

/// Is game currently running
#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameRunning {
    Yes,
    #[default]
    No,
}

impl GameRunning {
    pub fn is_yes(self) -> bool {
        match self {
            GameRunning::Yes => true,
            GameRunning::No => false,
        }
    }
}

/// Commands sent from user interface
#[derive(Event, Debug)]
pub enum GameCommand {
    /// Start the game
    Start {
        level_id: String,
    },

    Respawn,

    /// Delete all objects and stop the game
    Exit,
}

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameRunning>()
            .add_event::<GameCommand>()
            .add_systems(
                Update,
                execute_game_command.run_if(on_event::<GameCommand>()),
            );
    }
}

fn execute_game_command(
    mut game_commands: EventReader<GameCommand>,
    mut level_commands: EventWriter<LevelCommand>,
    is_running: Res<State<GameRunning>>,
    mut game_running: ResMut<NextState<GameRunning>>,
) {
    if let Some(command) = game_commands.read_single("execute_game_command") {
        info!("execute_game_command: {command:?}");

        let is_running = is_running.is_yes();

        match command {
            GameCommand::Start { level_id } => {
                if is_running {
                    error!("game already running");
                    return;
                }

                level_commands.send(LevelCommand::Load(level_id.clone()));
                game_running.set(GameRunning::Yes);
            }
            GameCommand::Respawn => {
                if !is_running {
                    error!("game not running");
                    return;
                }

                level_commands.send(LevelCommand::Reload);
            }
            GameCommand::Exit => {
                if !is_running {
                    error!("game already not running");
                    return;
                }

                level_commands.send(LevelCommand::Unload);
                game_running.set(GameRunning::No);
            }
        }
    }
}
