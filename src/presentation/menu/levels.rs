use super::states::MenuState;
use super::ui_const::UiConst;
use crate::app::scores::Scores;
use crate::app::settings::AppSettings;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::level::current::LevelCommand;
use crate::gameplay::master::level_progress::GotoNextLevel;
use crate::gameplay::master::level_progress::LevelList;
use crate::utils::bevy::misc_utils::ExtendedTimer;
use crate::utils::bevy_egui::*;
use crate::utils::math_algorithms::map_linear_range;
use bevy::prelude::*;
use std::time::Duration;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                level_select.run_if(in_state(MenuState::LevelSelect)),
                level_loading,
            ),
        );
    }
}

fn level_select(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    mut game_commands: EventWriter<GameCommand>,
    scores: Res<Scores>,
    levels: Res<LevelList>,
    settings: Res<AppSettings>,
) {
    let level_editor = settings.debug.developer_mode;

    EguiPopup {
        name: "level_select",
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for id in levels.all() {
                let name = levels.name(id);
                let completed = scores.completed_levels.contains(id);

                ui.horizontal(|ui| {
                    if level_editor {
                        if ui.button("EDIT").clicked() {
                            next_state.set(MenuState::LevelEditor);
                            game_commands.send(GameCommand::Start {
                                level_id: id.clone(),
                            });
                        }
                    }

                    if ui.button(name).clicked() {
                        next_state.set(MenuState::None);
                        game_commands.send(GameCommand::Start {
                            level_id: id.clone(),
                        });
                    }

                    if completed {
                        ui.label("(Completed)");
                    }
                });
            }
        });
    });
}

#[derive(Default)]
struct LoadingState {
    timer: Timer,
    id: String,
    gone_to: bool,
}

fn level_loading(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    mut game_commands: EventWriter<GameCommand>,
    mut res_state: Local<Option<LoadingState>>,
    mut next_level: EventReader<GotoNextLevel>,
    time: Res<Time<Real>>,
    mut level_commands: EventWriter<LevelCommand>,
    levels: Res<LevelList>,
    ui_const: UiConst,
) {
    let fade_duration = Duration::from_millis(1500);
    let text_size = 80. * ui_const.scale();

    if let Some(next) = next_level.read().last() {
        match &next.id {
            Some(id) => {
                *res_state = Some(LoadingState {
                    timer: Timer::once(fade_duration),
                    id: id.clone(),
                    gone_to: false,
                });
                next_state.set(MenuState::LevelLoading);
            }
            None => {
                next_state.set(MenuState::Outro);
                game_commands.send(GameCommand::Exit);
            }
        }
    }

    if let Some(state) = res_state.as_mut() {
        state.timer.tick(time.delta());

        if state.timer.finished() {
            *res_state = None;
            next_state.set(MenuState::None);
            return;
        }

        if state.timer.elapsed() >= fade_duration / 2 && !state.gone_to {
            state.gone_to = true;

            level_commands.send(LevelCommand::Load(state.id.clone()));
        }

        let t = state.timer.t_elapsed();
        let brek = 0.3;
        let alpha = if t < brek {
            map_linear_range(t, 0., brek, 0., 1., true)
        } else if t < 1. - brek {
            1.
        } else {
            map_linear_range(t, 1. - brek, 1., 1., 0., true)
        };

        EguiPopup {
            name: "level_loading background",
            anchor: egui::Align2::LEFT_TOP,
            order: egui::Order::Foreground,
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

        EguiPopup {
            name: "level_loading name",
            order: egui::Order::Tooltip,
            background: false,
            interactable: false,
            ..default()
        }
        .show(egui_ctx.ctx_mut(), |ui| {
            // level name
            ui.visuals_mut().override_text_color = Color::WHITE.with_a(alpha).to_egui().into();
            ui.label(egui::RichText::new(levels.name(&state.id)).size(text_size));
        });
    }
}
