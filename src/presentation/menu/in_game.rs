use super::level_editor::EditorEnabled;
use super::states::MenuState;
use crate::app::actions::action_axis_xy;
use crate::app::actions::ActionPrompt;
use crate::app::actions::PlayerActions;
use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::objects::player::Player;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (draw_hud, player_input.before(MechanicSet::Input))
                    .run_if(in_state(MenuState::None).and_then(in_state(EditorEnabled::No))),
                toggle_help_menu,
                draw_help_menu.run_if(in_state(MenuState::Help)),
            ),
        );
    }
}

fn draw_hud(mut egui_ctx: EguiContexts, level: Res<CurrentLevel>) {
    EguiPopup {
        name: "draw_hud",
        anchor: egui::Align2::CENTER_TOP,
        interactable: false,
        background: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        let name = &level.data.name;
        ui.label(format!("Level: {name}. HP: 100%"));
    });
}

fn player_input(
    actions: Res<ActionState<PlayerActions>>,
    mut players: Query<(&mut RotateToTarget, &mut MovementController, &mut Player)>,
) {
    for (mut rotate, mut mvmt, mut _player) in players.iter_mut() {
        let dir = action_axis_xy(&actions, PlayerActions::Movement);

        if dir.length() > 0.01 {
            rotate.target_dir = dir;
        }

        mvmt.target_dir = dir;
    }
}

fn toggle_help_menu(
    actions: Res<ActionState<PlayerActions>>,
    state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    if actions.just_pressed(PlayerActions::ToggleHelp) {
        next_state.set(match state.get() {
            MenuState::None => MenuState::Help,
            MenuState::Help => MenuState::None,
            _ => return,
        });
    }
}

fn draw_help_menu(mut egui_ctx: EguiContexts, prompt: ActionPrompt<PlayerActions>) {
    EguiPopup {
        name: "draw_help_menu",
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Objective");
        ui.label("1. Overload alien network");
        ui.label("2. Reach the elevator");
        ui.label("");

        ui.heading("Controls");
        egui::Grid::new("Controls").show(ui, |ui| {
            ui.label("Walk");
            ui.label(prompt.get(PlayerActions::Movement));
            ui.end_row();

            ui.label("Pick up");
            ui.label(prompt.get(PlayerActions::PickUp));
            ui.end_row();

            ui.label("Throw");
            ui.label(prompt.get(PlayerActions::Throw));
            ui.end_row();
        });
    });
}
