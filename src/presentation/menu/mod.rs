use bevy::prelude::*;

mod loading;
mod log;
mod main;

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((loading::LoadingPlugin, main::MainPlugin, log::LogPlugin));
    }
}
