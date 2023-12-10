use crate::app::actions::AppActions;
use crate::app::scores::Scores;
use crate::app::settings::AppSettings;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::game_states::GameRunning;
use crate::gameplay::master::level_progress::LevelList;
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
    LevelSelect,
    Settings,
    LevelEditor,

    ModalMessage,
    Help,
    LevelLoading,
    Intro,
    Outro,
}

#[derive(Event, Default)]
pub struct CloseMenu;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_event::<CloseMenu>()
            .add_systems(
                Last,
                update_game_controls.run_if(state_changed::<MenuState>()),
            )
            .add_systems(
                PostUpdate,
                (
                    on_load_complete.run_if(on_event::<LoadedTrackedAssets>()),
                    on_actions,
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
    levels: Res<LevelList>,
) {
    if settings.debug.quick_edit || settings.debug.quick_start {
        let level_id = scores
            .last_level
            .as_ref()
            .map(|level| level.id.clone())
            .unwrap_or(levels.first());

        next_state.set(if settings.debug.quick_edit {
            MenuState::LevelEditor
        } else {
            MenuState::None
        });

        game_commands.send(GameCommand::Start { level_id });
    } else {
        next_state.set(MenuState::MainMenu);
    }
}

fn on_actions(
    actions: Res<ActionState<AppActions>>,
    mut close_menu: EventReader<CloseMenu>,
    state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<MenuState>>,
    game_running: Res<State<GameRunning>>,
) {
    if actions.just_pressed(AppActions::CloseMenu) || close_menu.read().count() != 0 {
        match state.get() {
            MenuState::Startup => (),
            MenuState::None => next_state.set(MenuState::MainMenu),
            MenuState::MainMenu => match game_running.get() {
                GameRunning::Yes => next_state.set(MenuState::None),
                GameRunning::No => (),
            },
            MenuState::LevelSelect => next_state.set(MenuState::MainMenu),
            MenuState::Settings => next_state.set(MenuState::MainMenu),
            MenuState::LevelEditor => (),
            MenuState::ModalMessage => match game_running.get() {
                GameRunning::Yes => next_state.set(MenuState::None),
                GameRunning::No => next_state.set(MenuState::MainMenu),
            },
            MenuState::Help => next_state.set(MenuState::None),
            MenuState::LevelLoading => (),
            MenuState::Intro => next_state.set(MenuState::None),
            MenuState::Outro => next_state.set(MenuState::MainMenu),
        };
    }

    if actions.just_pressed(AppActions::LevelEditor) {
        match state.get() {
            MenuState::None => {
                if game_running.get().is_yes() {
                    next_state.set(MenuState::LevelEditor)
                }
            }
            MenuState::LevelEditor => next_state.set(MenuState::None),
            _ => (),
        }
    }
}
