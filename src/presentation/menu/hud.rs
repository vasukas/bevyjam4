use crate::app::actions::PlayerActions;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use super::level_editor::EditorEnabled;
use super::states::MenuState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            draw_hud,
            player_input
        ).run_if(in_state(MenuState::None).and_then(in_state(EditorEnabled::No))));
    }
}

fn draw_hud(

) {

}

fn player_input(
    actions: Res<ActionState<PlayerActions>>,
) {

}
