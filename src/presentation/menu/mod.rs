use bevy::prelude::*;

mod debug;
mod hud;
mod level_editor;
mod log;
mod main;
mod settings;
mod startup;
mod states;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            level_editor::LevelEditorPlugin,
            debug::DebugPlugin,
            log::LogPlugin,
            main::MainPlugin,
            settings::SettingsPlugin,
            states::StatesPlugin,
            startup::StartupPlugin,
            hud::HudPlugin,
        ));
    }
}
