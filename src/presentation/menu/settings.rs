use super::states::MenuState;
use crate::app::settings::AppSettings;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_settings_menu.run_if(in_state(MenuState::Settings)),
        );
    }
}

fn draw_settings_menu(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    mut settings_res: ResMut<AppSettings>,
    mut new_ui_scale: Local<Option<f32>>,
) {
    EguiPopup {
        name: "draw_settings_menu",
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            let settings = settings_res.bypass_change_detection();
            let mut changed = false;

            ui.group(|ui| {
                ui.strong("Graphics");

                let mut ui_scale = settings.graphics.ui_scale;
                if ui
                    .add(
                        egui::Slider::new(&mut ui_scale, 0.1..=10.)
                            .text("UI scale")
                            .clamp_to_range(false)
                            .prefix("x"),
                    )
                    .changed()
                {
                    *new_ui_scale = Some(ui_scale);
                }
                if ui.button("Apply UI scale").clicked() {
                    settings.graphics.ui_scale = ui_scale;
                    changed = true;
                }

                changed |= ui
                    .checkbox(&mut settings.graphics.fullscreen, "Fullscreen")
                    .changed();
            });

            if changed {
                settings_res.set_changed();
            }
        });

        if ui.button("Back to menu").clicked() {
            next_state.set(MenuState::MainMenu);
        }
    });
}
