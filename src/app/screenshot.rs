use super::actions::AppActions;
use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::action_state::ActionState;

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, make_screenshot);
    }
}

fn make_screenshot(
    actions: Res<ActionState<AppActions>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        error!("can't make screenshot: no single PrimaryWindow");
        return;
    };

    if actions.just_pressed(AppActions::Screenshot) {
        let _ = std::fs::create_dir_all("user/screenshots/");

        // current local time formatted like "2023-11-01_23-59-59_999" (with milliseconds!)
        let filename = chrono::prelude::Local::now()
            .format("user/screenshots/%F_%H-%M-%S_%3f.png")
            .to_string();

        let _ = screenshot_manager.save_screenshot_to_disk(window, filename);
    }
}
