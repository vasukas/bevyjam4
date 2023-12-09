use super::states::MenuState;
use crate::app::scores::Scores;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::game_states::GameRunning;
use crate::gameplay::master::level_progress::LevelList;
use crate::utils::bevy_egui::*;
use bevy::app::AppExit;
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
                crab,
            ),
        );
    }
}

fn draw_main_menu(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    game_running: Res<State<GameRunning>>,
    mut game_commands: EventWriter<GameCommand>,
    scores: Res<Scores>,
    mut exit: EventWriter<AppExit>,
    levels: Res<LevelList>,
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
                }
                if ui.button("Restart").clicked() {
                    next_state.set(MenuState::None);
                    game_commands.send(GameCommand::Respawn);
                }
            }
            GameRunning::No => {
                if let Some(level) = &scores.last_level {
                    if ui
                        .button(format!("Continue: {}", levels.name(&level.id)))
                        .clicked()
                    {
                        next_state.set(MenuState::None);
                        game_commands.send(GameCommand::Start {
                            level_id: level.id.clone(),
                        });
                    }
                }

                if ui.button("New game").clicked() {
                    next_state.set(MenuState::None);
                    game_commands.send(GameCommand::Start {
                        level_id: levels.first(),
                    });
                }

                if ui.button("Select level").clicked() {
                    next_state.set(MenuState::LevelSelect);
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

        if game_running.get().is_yes() {
            ui.label("");

            if ui.button("Exit to main menu").clicked() {
                game_commands.send(GameCommand::Exit);
            }
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            ui.label("");
        }

        #[cfg(not(target_arch = "wasm32"))] // there is no point in exiting in wasm
        if ui.button("Exit to desktop").clicked() {
            exit.send_default();
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

fn crab(mut egui_ctx: EguiContexts, state: Res<State<MenuState>>, mut hovered: Local<bool>) {
    let show = match state.get() {
        MenuState::Startup | MenuState::MainMenu | MenuState::LevelSelect | MenuState::Settings => {
            true
        }
        MenuState::None
        | MenuState::LevelEditor
        | MenuState::ModalMessage
        | MenuState::Help
        | MenuState::LevelLoading => false,
    };

    if show {
        EguiPopup {
            name: "crab",
            anchor: egui::Align2::RIGHT_BOTTOM,
            order: egui::Order::Tooltip,
            background: false,
            ..default()
        }
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                let text = match *hovered {
                    true => "()_^^_()",
                    false => "()_.._()",
                };
                *hovered = ui.label(egui::RichText::new(text).monospace()).hovered();
            });
        });
    }
}
