use super::states::MenuState;
use crate::app::settings::AppSettings;
use crate::utils::bevy_egui::*;
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NewUiScale>()
            .add_systems(OnEnter(MenuState::Settings), update_ui_scale)
            .add_systems(
                Update,
                draw_settings_menu.run_if(in_state(MenuState::Settings)),
            );
    }
}

#[derive(Resource, Default)]
struct NewUiScale(f32);

fn update_ui_scale(settings: Res<AppSettings>, mut new_ui_scale: ResMut<NewUiScale>) {
    new_ui_scale.0 = settings.graphics.ui_scale;
}

fn draw_settings_menu(
    mut egui_ctx: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    mut settings_res: ResMut<AppSettings>,
    mut new_ui_scale: ResMut<NewUiScale>,
) {
    EguiPopup {
        name: "draw_settings_menu",
        ..default()
    }
    .show(egui_ctx.ctx_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            let settings = settings_res.bypass_change_detection();
            let mut changed = false;

            if settings.debug.developer_mode {
                ui.group(|ui| {
                    ui.strong("Debug");

                    changed |= ui
                        .checkbox(&mut settings.debug.show_fps, "Show FPS counter")
                        .changed();

                    changed |= ui
                        .checkbox(&mut settings.debug.show_physics, "Debug physics render")
                        .changed();

                    changed |= ui
                        .checkbox(&mut settings.debug.quick_start, "On startup: Continue")
                        .changed();

                    changed |= ui
                        .checkbox(
                            &mut settings.debug.quick_edit,
                            "On startup: Continue & Level Editor",
                        )
                        .changed();
                });
            }

            ui.group(|ui| {
                ui.strong("Graphics");

                ui.add(
                    egui::Slider::new(&mut new_ui_scale.0, 0.1..=10.)
                        .text("UI scale")
                        .clamp_to_range(false)
                        .prefix("x"),
                );
                if ui.button("Apply UI scale").clicked() {
                    settings.graphics.ui_scale = new_ui_scale.0;
                    changed = true;
                }

                changed |= ui
                    .checkbox(&mut settings.graphics.fullscreen, "Fullscreen")
                    .changed();

                changed |= ui
                    .checkbox(&mut settings.graphics.shadows, "Shadows")
                    .changed();

                changed |= ui
                    .checkbox(&mut settings.graphics.starfield, "Background")
                    .changed();
            });

            changed |= ui
                .checkbox(&mut settings.debug.developer_mode, "Developer mode")
                .changed();

            if changed {
                settings_res.set_changed();
            }
        });

        if ui.button("Back to menu").clicked() {
            next_state.set(MenuState::MainMenu);
        }
    });
}
