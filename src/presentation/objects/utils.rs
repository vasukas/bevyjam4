use crate::gameplay::master::level::spawn::GameObject;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

/// Rotate normal 3D coordinates to pseudo-2D ones
pub fn rotate_3to2() -> Quat {
    let r1 = Quat::from_rotation_z(FRAC_PI_2);
    let r2 = Quat::from_rotation_y(FRAC_PI_2);
    r2 * r1
}

/// Rotate normal 3D coordinates to pseudo-2D ones
pub fn rotate_3to2_tr() -> Transform {
    Transform::from_rotation(rotate_3to2())
}

/// The main camera
#[derive(Bundle)]
pub struct WorldCameraBundle {
    pub name: Name,
    pub game_object: GameObject,
    pub camera: Camera3dBundle,
    pub bloom: BloomSettings,
    pub fxaa: Fxaa,
}

impl WorldCameraBundle {
    pub const VERTICAL_SIZE: f32 = 12.; // viewport height in world units

    pub fn new(name: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        let vertical_fov = 28.8_f32.to_radians(); // 70 mm
        let vertical_size = Self::VERTICAL_SIZE;
        let distance = (vertical_size / 2.) / (vertical_fov / 2.).tan();

        Self {
            name: Name::new(name),
            game_object: GameObject,
            camera: Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                projection: PerspectiveProjection {
                    fov: vertical_fov,
                    ..default()
                }
                .into(),
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::rgb(0., 0.04, 0.09)),
                    ..default()
                },
                tonemapping: Tonemapping::AgX,
                // emulate 2D camera
                transform: Transform::from_xyz(0., 0., distance).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            bloom: BloomSettings::OLD_SCHOOL,
            fxaa: default(),
        }
    }
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, fix_lights);
    }
}

// TODO: file bug report; shadows visibility is not updated for lights (at all?),
// so there are no shadows if lights are spawned before the camera
// (this issue? https://github.com/bevyengine/bevy/issues/8535)
fn fix_lights(new_cameras: Query<(), Added<Camera3d>>, mut lights: Query<&mut PointLight>) {
    if !new_cameras.is_empty() {
        for mut light in lights.iter_mut() {
            if light.shadows_enabled {
                light.set_changed();
            }
        }
    }
}
