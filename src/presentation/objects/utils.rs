use crate::gameplay::master::level::spawn::GameObject;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use std::f32::consts::FRAC_2_PI;

/// Rotate normal 3D coordinates to pseudo-2D ones
pub fn rotate_3to2() -> Quat {
    let r1 = Quat::from_rotation_z(FRAC_2_PI);
    let r2 = Quat::from_rotation_y(FRAC_2_PI);
    r2 * r1
}

/// Rotate normal 3D coordinates to pseudo-2D ones
pub fn rotate_3to2_tr() -> Transform {
    Transform::from_rotation(rotate_3to2())
}

#[derive(Bundle)]
pub struct WorldCameraBundle {
    pub name: Name,
    pub game_object: GameObject,
    pub camera: Camera3dBundle,
    pub bloom: BloomSettings,
    pub fxaa: Fxaa,
}

impl WorldCameraBundle {
    pub fn new(name: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        let vertical_fov = 28.8_f32.to_radians(); // 70 mm
        let vertical_size = 10.; // viewport height in world units
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
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
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
    fn build(&self, _app: &mut App) {
        //
    }
}
