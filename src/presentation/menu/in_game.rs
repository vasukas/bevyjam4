use super::level_editor::EditorEnabled;
use super::states::MenuState;
use crate::app::actions::action_axis_xy;
use crate::app::actions::ActionPrompt;
use crate::app::actions::PlayerActions;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::objects::player::Player;
use crate::gameplay::objects::player::PLAYER_HEALTH;
use crate::gameplay::utils::rotate_to_target;
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
                (
                    draw_hud,
                    player_input
                        .before(MechanicSet::Input)
                        .before(rotate_to_target),
                )
                    .run_if(in_state(MenuState::None).and_then(in_state(EditorEnabled::No))),
                toggle_help_menu,
                draw_help_menu.run_if(in_state(MenuState::Help)),
            ),
        );
    }
}

fn draw_hud(
    mut egui_ctx: EguiContexts,
    level: Res<CurrentLevel>,
    player: Query<&Health, (With<Player>, Without<Dead>)>,
) {
    let Ok(health) = player.get_single() else { return; };

    EguiPopup {
        name: "draw_hud",
        anchor: egui::Align2::CENTER_TOP,
        interactable: false,
        background: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        let style = ui.style_mut();
        style.visuals.window_fill = Color::BLACK.with_a(0.7).to_egui();
        style.visuals.window_stroke = egui::Stroke::NONE; // no border
        style.visuals.popup_shadow = egui::epaint::Shadow::NONE;

        egui::Frame::popup(style).show(ui, |ui| {
            let hp = (health.value as f32 / PLAYER_HEALTH as f32 * 100.) as i32;
            ui.visuals_mut().override_text_color = match hp {
                _ if hp < 50 => Color::ORANGE_RED,
                _ if hp < 75 => Color::YELLOW,
                _ => Color::WHITE,
            }
            .to_egui()
            .into();
            ui.label(format!("HP: {hp:3}"));

            ui.visuals_mut().override_text_color = egui::Color32::from_gray(192).into();
            ui.small(format!("\"{}\"", level.data.name));
        });
    });
}

fn player_input(
    actions: Res<ActionState<PlayerActions>>,
    mut players: Query<(&mut RotateToTarget, &mut MovementController, &mut Player), Without<Dead>>,
    mut game_commands: EventWriter<GameCommand>,
) {
    for (mut rotate, mut mvmt, mut player) in players.iter_mut() {
        let dir = action_axis_xy(&actions, PlayerActions::Movement);

        if dir.length() > 0.01 {
            rotate.target_dir = dir;
            player.input_walking = true;
        }

        mvmt.target_dir = dir;
    }

    if actions.just_pressed(PlayerActions::Restart) {
        game_commands.send(GameCommand::Respawn);
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
        ui.label("1. Overload aliens!");
        ui.label("2. Reach elevator");
        ui.label("");

        ui.heading("Controls");
        egui::Grid::new("Controls").show(ui, |ui| {
            ui.label("Walk");
            ui.label(prompt.get(PlayerActions::Movement));
            ui.end_row();

            ui.label("Restart levle");
            ui.label(prompt.get(PlayerActions::Restart));
            ui.end_row();
        });
    });
}
