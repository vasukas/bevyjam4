use super::level_editor::EditorEnabled;
use super::states::MenuState;
use crate::app::actions::action_axis_xy;
use crate::app::actions::PlayerActions;
use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::mechanics::movement::MovementController;
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
            (draw_hud, player_input)
                .run_if(in_state(MenuState::None).and_then(in_state(EditorEnabled::No))),
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
