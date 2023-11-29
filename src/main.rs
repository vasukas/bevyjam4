#![cfg_attr(target_arch = "wasm32", allow(unused))] // wasm doesn't use some stuff

use bevy::prelude::*;

mod app;
mod gameplay;
mod presentation;
mod tmp;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            utils::plugins::log_plugin::LogPlugin::default(),
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (1280., 720.).into(),
                        title: "Jam Game".to_string(),
                        // resizable must be false for fullscreen toggle to work
                        ..default()
                    }),
                    ..default()
                })
                .disable::<bevy::log::LogPlugin>(),
            bevy_egui::EguiPlugin,
            utils::plugins::UtilPlugins,
            app::AppPlugins,
            gameplay::GameplayPlugins,
            presentation::PresentationPlugins,
            tmp::TmpPlugin,
        ))
        .run()
}
