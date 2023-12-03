use super::states::MenuState;
use crate::utils::bevy_egui::*;
use crate::utils::misc_utils::DurationDivF32;
use crate::utils::plugins::load_assets::LoadedTrackedAssets;
use bevy::prelude::*;
use std::time::Duration;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_startup_menu.run_if(in_state(MenuState::Startup)),
                on_loading_done.run_if(on_event::<LoadedTrackedAssets>()),
            ),
        );
    }
}

fn draw_startup_menu(mut egui_ctx: EguiContexts, time: Res<Time<Real>>) {
    EguiPopup {
        name: "loading_menu",
        anchor: egui::Align2::LEFT_TOP,
        order: egui::Order::Foreground,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        // number of dots in label
        let dot_duration = Duration::from_millis(250);
        let max_dot_count = 3;
        let dot_count = time.elapsed().div_dur_f32(dot_duration) as i32 % (max_dot_count + 1);

        // label
        let text = format!(
            "Loading{}",
            ".".repeat(dot_count.try_into().unwrap_or_default())
        );

        // allocate entire screen
        let size = ui.available_size();
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

        // black background
        ui.painter()
            .rect_filled(rect, egui::Rounding::ZERO, egui::Color32::BLACK);

        // white text in center
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            default(),
            egui::Color32::WHITE,
        );
    });
}

fn on_loading_done(time: Res<Time<Real>>) {
    info!("Loaded all assets at {:.3} seconds", time.elapsed_seconds());
}
