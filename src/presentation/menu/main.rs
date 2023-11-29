use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiSettings;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_egui_size);

        // can't exit app in web
        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Update, exit_app_on_ctrl_q);
    }
}

const UI_SCALE: f32 = 2.; // TODO: should be a setting

fn set_egui_size(
    mut egui_settings: ResMut<EguiSettings>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // TODO: should not panic here
    let window = window
        .get_single()
        .expect("PrimaryWindow should already exist!");
    egui_settings.scale_factor = UI_SCALE as f64 / window.scale_factor();
}

#[cfg(not(target_arch = "wasm32"))]
fn exit_app_on_ctrl_q(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Q) && keys.pressed(KeyCode::ControlLeft) {
        exit.send_default()
    }
}
