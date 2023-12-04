use bevy::prelude::*;

mod debug;
mod in_game;
mod level_editor;
mod log;
mod main;
mod messages;
mod settings;
mod startup;
mod states;

pub use messages::{DelayedMessage, Message, MessageType};

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
            in_game::HudPlugin,
            messages::MessagesPlugin,
        ));
    }
}
