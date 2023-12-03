use crate::utils::bevy_egui::*;
use bevy::diagnostic::*;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, show_fps_count);
    }
}

fn show_fps_count(mut egui_ctx: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    EguiPopup {
        name: "show_fps_count",
        background: false,
        anchor: egui::Align2::RIGHT_TOP,
        interactable: false,
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
        let average = diag.average().unwrap_or_default();

        ui.style_mut().wrap = Some(false); // otherwise it will split text into like a dozen lines
        ui.label(format!("FPS: {average:6.2} (avg 20 frames)"));
    });
}
