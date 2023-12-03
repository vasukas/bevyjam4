use super::states::MenuState;
use super::states::PreviousMenu;
use crate::app::scores::Scores;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::game_states::GameRunning;
use crate::gameplay::master::level::data::FIRST_LEVEL_ID;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_main_menu.run_if(in_state(MenuState::MainMenu)),
                draw_menu_background.run_if(
                    // draw background only if in menu and game is running,
                    // but don't draw it in level editor menu
                    not(in_state(MenuState::None))
                        .and_then(in_state(GameRunning::Yes))
                        .and_then(not(in_state(MenuState::LevelEditor))),
                ),
            ),
        );
    }
}

fn draw_main_menu(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    game_running: Res<State<GameRunning>>,
    mut game_commands: EventWriter<GameCommand>,
    mut prev_menu: ResMut<PreviousMenu>,
    scores: Res<Scores>,
) {
    EguiPopup {
        name: "draw_main_menu",
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        match game_running.get() {
            GameRunning::Yes => {
                if ui.button("Back to game").clicked() {
                    next_state.set(MenuState::None);
                    prev_menu.0 = MenuState::MainMenu;
                }
                if ui.button("Restart").clicked() {
                    next_state.set(MenuState::None);
                    game_commands.send(GameCommand::Respawn);
                }
            }
            GameRunning::No => {
                if let Some(level) = &scores.level {
                    if ui.button(format!("Continue: {}", level.name)).clicked() {
                        next_state.set(MenuState::None);
                        game_commands.send(GameCommand::Start {
                            level_id: level.id.clone(),
                        });
                    }
                }

                if ui.button("New game").clicked() {
                    next_state.set(MenuState::None);
                    game_commands.send(GameCommand::Start {
                        level_id: FIRST_LEVEL_ID.to_string(),
                    });
                }
            }
        }

        ui.label("");

        if ui.button("Settings").clicked() {
            next_state.set(MenuState::Settings);
        }

        #[cfg(not(target_arch = "wasm32"))] // levels can't be saved on wasm
        if game_running.get().is_yes() {
            if ui.button("Edit level").clicked() {
                next_state.set(MenuState::LevelEditor);
            }
        }

        ui.label("");

        if game_running.get().is_yes() {
            if ui.button("Exit to main menu").clicked() {
                game_commands.send(GameCommand::Exit);
            }
        }
    });
}

fn draw_menu_background(mut egui_ctx: EguiContexts) {
    let alpha = 0.7;

    EguiPopup {
        name: "draw_menu_background",
        anchor: egui::Align2::LEFT_TOP,
        order: egui::Order::Background,
        background: false,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        // allocate entire screen
        let size = ui.available_size();
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

        // black background
        ui.painter().rect_filled(
            rect,
            egui::Rounding::ZERO,
            Color::BLACK.with_a(alpha).to_egui(),
        );
    });
}
