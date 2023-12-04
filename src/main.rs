use bevy::log::Level;
use bevy::prelude::*;

mod app;
mod gameplay;
mod presentation;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            // TODO: remove log filtering
            utils::plugins::log_plugin::LogPlugin::default().with_filter("bevy_gltf", Level::ERROR),
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (1280., 720.).into(),
                        title: "Alien Overload".to_string(),
                        // resizable must be false for fullscreen toggle to work
                        ..default()
                    }),
                    ..default()
                })
                .disable::<bevy::log::LogPlugin>(),
            bevy_egui::EguiPlugin,
            utils::plugins::UtilPlugin,
            app::AppPlugin,
            gameplay::GameplayPlugin,
            presentation::PresentationPlugin,
        ))
        .insert_resource(GizmoConfig {
            depth_bias: -1., // always render in front
            ..default()
        })
        .add_systems(Update, exit_on_ctrl_q)
        .run()
}

fn exit_on_ctrl_q(keys: Res<Input<KeyCode>>, mut exit: EventWriter<bevy::app::AppExit>) {
    if keys.pressed(KeyCode::Q) && keys.pressed(KeyCode::ControlLeft) {
        exit.send_default()
    }
}
