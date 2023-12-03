use crate::app::actions::AppActions;
use crate::app::scores::Scores;
use crate::app::settings::AppSettings;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::game_states::GameRunning;
use crate::gameplay::master::time_master::TimeMaster;
use crate::utils::plugins::load_assets::LoadedTrackedAssets;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum MenuState {
    /// Initial states - loading assets etc
    #[default]
    Startup,
    /// No menu - game is running
    None,

    MainMenu,
    Settings,
    LevelEditor,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .init_resource::<PreviousMenu>()
            .add_systems(
                Last,
                update_game_controls.run_if(state_changed::<MenuState>()),
            )
            .add_systems(
                PostUpdate,
                (
                    on_load_complete.run_if(on_event::<LoadedTrackedAssets>()),
                    on_back,
                ),
            );
    }
}

fn update_game_controls(state: Res<State<MenuState>>, mut time: ResMut<TimeMaster>) {
    let in_menu = match state.get() {
        MenuState::None => false,
        _ => true,
    };

    time.in_menu = in_menu;
}

fn on_load_complete(
    mut next_state: ResMut<NextState<MenuState>>,
    settings: Res<AppSettings>,
    mut game_commands: EventWriter<GameCommand>,
    scores: Res<Scores>,
) {
    if settings.debug.quick_edit || settings.debug.quick_start {
        if let Some(level) = &scores.last_level {
            next_state.set(if settings.debug.quick_edit {
                MenuState::LevelEditor
            } else {
                MenuState::None
            });

            game_commands.send(GameCommand::Start {
                level_id: level.id.clone(),
            });
        }
    } else {
        next_state.set(MenuState::MainMenu);
    }
}

/// Last state menu was in
#[derive(Resource)]
pub struct PreviousMenu(pub MenuState);

impl Default for PreviousMenu {
    fn default() -> Self {
        Self(MenuState::MainMenu)
    }
}

fn on_back(
    actions: Res<ActionState<AppActions>>,
    state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut previous: ResMut<PreviousMenu>,
    game_running: Res<State<GameRunning>>,
) {
    if actions.just_pressed(AppActions::MenuBack) {
        let new_state = match state.get() {
            MenuState::Startup => {
                return;
            }
            MenuState::None => previous.0,
            _ => match game_running.get() {
                GameRunning::Yes => {
                    previous.0 = *state.get();
                    MenuState::None
                }
                GameRunning::No => return,
            },
        };

        next_state.set(new_state);
    }
}
