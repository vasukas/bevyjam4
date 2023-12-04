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
        let dir = "user/screenshots";
        let _ = std::fs::create_dir_all(dir);

        let mut filename;
        let mut index = 0;
        loop {
            assert!(index < 100);

            index += 1;
            filename = format!("{dir}/{index:02}.png");

            if std::fs::metadata(&filename).is_err() {
                break;
            }
        }

        let _ = screenshot_manager.save_screenshot_to_disk(window, filename);
    }
}
