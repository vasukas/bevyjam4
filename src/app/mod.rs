use bevy::prelude::*;

pub mod actions;
pub mod scheduling;
pub mod scores;
pub mod settings;

#[cfg(not(target_arch = "wasm32"))]
mod screenshot;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            settings::SettingsPlugin,
            actions::ActionsPlugin,
            scheduling::SchedulingPlugin,
            scores::ScoresPlugin,
        ));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(screenshot::ScreenshotPlugin);
    }
}
