use crate::{
    app::assets::{AllAssetsState, AssetLoadInfo, LoadedAllAssets},
    utils::for_crate::std::DurationDivF32,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::time::Duration;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                loading_menu.run_if(in_state(AllAssetsState::StillLoading)),
                asset_errors_notification,
                log_load_finished_time.run_if(on_event::<LoadedAllAssets>()),
            ),
        );
    }
}

fn loading_menu(mut egui_ctx: EguiContexts, time: Res<Time<Real>>) {
    egui::Area::new("loading_menu")
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
        .order(egui::Order::Foreground)
        .interactable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            // number of dots in label
            let dot_duration = Duration::from_millis(250);
            let max_dot_count = 3;
            let dot_count =
                time.elapsed().div_duration_f32_fr(dot_duration) as i32 % (max_dot_count + 1);

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

fn asset_errors_notification(mut egui_ctx: EguiContexts, info: Res<AssetLoadInfo>) {
    if info.errors != 0 {
        egui::Area::new("asset_errors_notification")
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
            .order(egui::Order::Tooltip)
            .interactable(false)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.visuals_mut().override_text_color = egui::Color32::RED.into();
                ui.heading(format!("{} assets failed to load!", info.errors));
            });
    }
}

fn log_load_finished_time(time: Res<Time<Real>>) {
    info!("Loaded all assets at {:.3} seconds", time.elapsed_seconds());
}
