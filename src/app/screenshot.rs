use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, make_screenshot_on_key);
    }
}

fn make_screenshot_on_key(
    keys: Res<Input<KeyCode>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    window: Query<Entity, With<PrimaryWindow>>,
    userdata: Res<Userdata>,
) {
    if keys.just_pressed(KeyCode::F12) {
        let Ok(window) = window.get_single() else { return; };
        let Some(filename) = userdata.new_screenshot() else { return; };

        let _ = screenshot_manager.take_screenshot(window, move |image| {
            // save screenshot in detached thread
            std::thread::spawn(move || {
                match image.try_into_dynamic() {
                    Ok(image) => {
                        let image = image.to_rgb8(); // discard alpha
                        match image.save(filename) {
                            Ok(path) => debug!("screenshot saved: {path:?}"),
                            Err(e) => error!("screenshot: image.save: {e}"),
                        }
                    }
                    Err(e) => error!("screenshot: image.try_into_dynamic: {e}"),
                }
            });
        });
    }
}
