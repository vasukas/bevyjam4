pub mod assets;
pub mod screenshot;
pub mod settings;

use bevy::prelude::*;

pub struct AppPlugins;

impl Plugin for AppPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            settings::SettingsPlugin,
            screenshot::ScreenshotPlugin,
        ));
    }
}
