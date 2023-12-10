use super::level_editor::EditorEnabled;
use super::states::MenuState;
use super::ui_const::UiConst;
use crate::app::actions::action_axis_xy;
use crate::app::actions::ActionPrompt;
use crate::app::actions::AppActions;
use crate::app::actions::PlayerActions;
use crate::gameplay::master::game_states::GameCommand;
use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::master::level::data::HALF_TILE;
use crate::gameplay::master::level_progress::LevelList;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::mechanics::overload::Overload;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::objects::player::Player;
use crate::gameplay::objects::player::PLAYER_HEALTH;
use crate::gameplay::utils::rotate_to_target;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy_egui::*;
use crate::utils::math_algorithms::lerp;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use std::time::Duration;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                died_since,
                (
                    draw_hud,
                    death_screen,
                    player_input
                        .before(MechanicSet::Input)
                        .before(rotate_to_target),
                    draw_overload,
                )
                    .run_if(in_state(MenuState::None).and_then(in_state(EditorEnabled::No))),
                toggle_help_menu,
                draw_help_menu.run_if(in_state(MenuState::Help)),
            ),
        );
    }
}

#[derive(Component)]
struct DeadSince(Duration);

fn died_since(died: Query<Entity, Added<Dead>>, mut commands: Commands, time: Res<Time<Real>>) {
    for entity in died.iter() {
        commands.try_insert(entity, DeadSince(time.elapsed()));
    }
}

fn draw_hud(
    mut egui_ctx: EguiContexts,
    level: Res<CurrentLevel>,
    levels: Res<LevelList>,
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
            ui.small(format!("\"{}\"", levels.name(&level.id)));
        });
    });
}

fn death_screen(
    mut egui_ctx: EguiContexts,
    dead_player: Query<&DeadSince, With<Player>>,
    prompt: ActionPrompt<PlayerActions>,
    time: Res<Time<Real>>,
) {
    let fade_duration = Duration::from_millis(1000);

    if let Ok(dead_since) = dead_player.get_single() {
        let t_passed = (time.elapsed().saturating_sub(dead_since.0).as_secs_f32()
            / fade_duration.as_secs_f32())
        .min(1.);

        EguiPopup {
            name: "death_screen background",
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
                Color::BLACK.with_a(t_passed).to_egui(),
            );
        });

        EguiPopup {
            name: "death_screen text",
            order: egui::Order::Foreground,
            interactable: false,
            ..default()
        }
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(egui::RichText::new("GAME OVER").heading().strong());

            ui.label(format!(
                "Press {} to restart level",
                prompt.get(PlayerActions::Restart)
            ));
        });
    }
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

        player.input_fire = actions.pressed(PlayerActions::Fire);
        player.input_pull = actions.pressed(PlayerActions::Pull);
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

fn draw_help_menu(
    mut egui_ctx: EguiContexts,
    prompt: ActionPrompt<PlayerActions>,
    app_prompt: ActionPrompt<AppActions>,
) {
    EguiPopup {
        name: "draw_help_menu",
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Escape!");
        ui.label("1. Overload aliens");
        ui.label("2. Reach elevator");
        ui.label("");

        ui.heading("Controls");
        egui::Grid::new("Controls").show(ui, |ui| {
            ui.label(prompt.get(PlayerActions::Movement));
            ui.label("Walk");
            ui.end_row();

            ui.label(prompt.get(PlayerActions::Fire));
            ui.label("Emit fire");
            ui.end_row();

            ui.label(prompt.get(PlayerActions::Pull));
            ui.label("Hold to pull objects, release to push");
            ui.end_row();

            ui.label(prompt.get(PlayerActions::Restart));
            ui.label("Restart level");
            ui.end_row();

            ui.label(prompt.get(PlayerActions::ToggleHelp));
            ui.label("Toggle this window");
            ui.end_row();

            ui.label(app_prompt.get(AppActions::CloseMenu));
            ui.label("Toggle menu");
            ui.end_row();
        });
    });
}

fn draw_overload(
    mut egui_ctx: EguiContexts,
    enemies: Query<(&GlobalTransform, &Overload, Option<&DeadSince>)>,
    camera: Query<(&GlobalTransform, &Camera)>,
    ui_const: UiConst,
    time: Res<Time<Real>>,
) {
    let color = |t_overload| match t_overload {
        t if t < 0.25 => Color::RED,
        t if t < 0.66 => Color::ORANGE_RED,
        t if t >= 1. => Color::WHITE,
        _ => Color::YELLOW,
    };
    let text = |t_overload: f32| format!("{:2}% CPU", (t_overload * 100.).min(100.) as i32);

    let text_size = 10.;
    let slider_width = 20.;

    let offset = Vec2::new(HALF_TILE * 0.5, HALF_TILE);
    let approx_text_size = 0.7 * 7.;

    let post_death_duration = Duration::from_millis(1600);

    //

    let Ok((camera_transform, camera)) = camera.get_single() else { return; };

    let painter = egui_ctx.ctx_mut().debug_painter();
    let scale_factor = ui_const.egui_scale_factor();
    let ui_const = ui_const.scale();

    let font = egui::FontId::monospace(text_size * ui_const);
    let slider_width = slider_width * ui_const;
    let approx_text_size = text_size * ui_const * approx_text_size;

    for (pos, overload, dead_since) in enemies.iter() {
        let alpha = match dead_since {
            Some(since) => {
                let t_passed = time.elapsed().saturating_sub(since.0).as_secs_f32()
                    / post_death_duration.as_secs_f32();
                if t_passed >= 1. {
                    continue;
                }
                lerp(2., 0., t_passed).clamp(0., 1.)
            }
            None => 1.,
        };

        let pos = pos.translation().truncate() + offset / scale_factor;

        let Some(center) = camera.world_to_viewport(camera_transform, pos.extend(0.)) else { return; };
        let rect = egui::Rect::from_center_size(
            (center / scale_factor).to_egui_pos(),
            Vec2::new(slider_width + approx_text_size, text_size).to_egui(),
        );

        let t_overload = overload.current / overload.max;
        let color = color(t_overload).set_a(alpha).to_egui();

        let mut slider_rect = rect;
        slider_rect.set_width(slider_width * t_overload);

        painter.rect(slider_rect, egui::Rounding::ZERO, color, egui::Stroke::NONE);
        painter.text(
            rect.max,
            egui::Align2::RIGHT_BOTTOM,
            text(t_overload),
            font.clone(),
            color,
        );
    }
}
