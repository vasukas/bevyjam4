use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy_egui::EguiSettings;
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
    pub graphics: GraphicalSettings,
}

/// In-app log display
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LogSettings {
    /// Show all messages
    pub show_all: bool,

    /// Ignored if [`Self::show_all`] is `true`
    pub hide_errors: bool,
}

/// Debug options
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DebugSettings {
    pub developer_mode: bool,

    pub show_fps: bool,
    pub show_physics: bool,

    /// On startup go to play last played level
    pub quick_start: bool,

    /// On startup go to edit last played level
    pub quick_edit: bool,
}

/// Graphics
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct GraphicalSettings {
    /// How big UI is
    pub ui_scale: f32,

    /// Fullscreen window
    pub fullscreen: bool,

    pub shadows: bool,
}

impl Default for GraphicalSettings {
    fn default() -> Self {
        Self {
            ui_scale: 2.,
            fullscreen: false,
            shadows: true,
        }
    }
}

/// Marker for [`PointLight`] which should have shadows enabled.
/// Needed for toggling shadows via settings.
#[derive(Component)]
pub struct LightWithShadows;

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
                    apply_settings.run_if(resource_changed::<AppSettings>()),
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

fn apply_settings(
    settings: Res<AppSettings>,
    mut egui_settings: ResMut<EguiSettings>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    #[cfg(feature = "dev_build")] mut rapier: ResMut<bevy_rapier2d::render::DebugRenderContext>,
    mut point_lights: Query<&mut PointLight, With<LightWithShadows>>,
) {
    egui_settings.scale_factor = settings.graphics.ui_scale as f64;

    if let Ok(mut window) = window.get_single_mut() {
        window.mode = match settings.graphics.fullscreen {
            true => WindowMode::BorderlessFullscreen,
            false => WindowMode::Windowed,
        };
    }

    #[cfg(feature = "dev_build")]
    {
        rapier.enabled = settings.debug.show_physics;
    }

    for mut light in point_lights.iter_mut() {
        light.shadows_enabled = settings.graphics.shadows;
    }
}
