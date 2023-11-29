use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

/// Global application settings saved as userdata.
///
/// Loaded on startup, saved on changes (with small delay, to reduce number of writes
/// on continiously changing values; delay is reset on each change).
#[derive(Resource, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub log: LogSettings,
    pub debug: DebugSettings,
}

/// In-app log display
#[derive(Default, Serialize, Deserialize)]
pub struct LogSettings {
    pub show_all: bool,

    /// Ignored if [`Self::show_all`] is `true`
    pub show_errors: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DebugSettings {
    pub show_fps: bool,
}

const SAVE_DELAY: Duration = Duration::from_secs(1);
const USERDATA_NAME: &str = "settings";

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppSettings>()
            .add_systems(PreStartup, load_settings)
            .add_systems(
                Update,
                (
                    init_save_settings.run_if(resource_changed::<AppSettings>()),
                    save_settings.run_if(resource_exists::<SaveAt>()),
                ),
            );
    }
}

fn load_settings(mut settings: ResMut<AppSettings>, userdata: Res<Userdata>) {
    *settings = userdata.read_and_update(USERDATA_NAME);
}

#[derive(Resource)]
struct SaveAt(Duration);

fn init_save_settings(mut commands: Commands, time: Res<Time<Real>>) {
    commands.insert_resource(SaveAt(time.elapsed() + SAVE_DELAY));
}

fn save_settings(
    settings: Res<AppSettings>,
    userdata: Res<Userdata>,
    save_at: Res<SaveAt>,
    time: Res<Time<Real>>,
    mut commands: Commands,
) {
    if time.elapsed() >= save_at.0 {
        userdata.write(USERDATA_NAME, &*settings);
        commands.remove_resource::<SaveAt>();
    }
}
