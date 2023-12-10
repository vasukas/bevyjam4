use super::states::MenuState;
use crate::app::actions::ActionPrompt;
use crate::app::actions::AppActions;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;

pub struct InoutroPlugin;

impl Plugin for InoutroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (show_escape, intro).run_if(in_state(MenuState::Intro)),
                (show_escape, outro).run_if(in_state(MenuState::Outro)),
            ),
        );
    }
}

fn show_escape(mut egui_ctx: EguiContexts, app_prompt: ActionPrompt<AppActions>) {
    EguiPopup {
        name: "show_escape",
        anchor: egui::Align2::RIGHT_BOTTOM,
        background: false,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
            ui.label(format!(
                "Press {} to skip",
                app_prompt.get(AppActions::CloseMenu)
            ));
        });
    });
}

fn intro(mut egui_ctx: EguiContexts) {
    EguiPopup {
        name: "intro",
        background: false,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("INTRO");
    });
}

fn outro(mut egui_ctx: EguiContexts) {
    EguiPopup {
        name: "outro",
        background: false,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Congratulations!");
        ui.label("You've completed the game!\nThere was supposed to be nice victory screen, but I ran out of time\nC:");
    });
}
