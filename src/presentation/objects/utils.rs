use crate::gameplay::master::level::spawn::GameObject;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
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
    pub fn new(name: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        let vertical_fov = 28.8_f32.to_radians(); // 70 mm
        let vertical_size = 12.; // viewport height in world units
        let distance = (vertical_size / 2.) / (vertical_fov / 2.).tan();

        info!("camera distance: {distance}");

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

/// For procedurally-made [`StandardMaterial`]
#[derive(Resource, Default)]
pub struct MaterialCache {
    map: HashMap<ParticleMaterial, Handle<StandardMaterial>>,
}

impl MaterialCache {
    pub fn get(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        descr: ParticleMaterial,
    ) -> Handle<StandardMaterial> {
        self.map
            .entry(descr)
            .or_insert_with(|| {
                let color = match descr {
                    ParticleMaterial::Simple { color } => color,
                };
                materials.add(StandardMaterial {
                    base_color: color,
                    unlit: true,
                    alpha_mode: AlphaMode::Add,
                    ..default()
                })
            })
            .clone()
    }
}

/// Descriptor for [`MaterialCache`]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ParticleMaterial {
    /// Unlit, additive blending
    Simple { color: Color },
}

impl PartialEq for ParticleMaterial {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParticleMaterial::Simple { color }, ParticleMaterial::Simple { color: color2 }) => {
                color_as_u64(color) == color_as_u64(color2)
            }
        }
    }
}

impl Eq for ParticleMaterial {}

impl std::hash::Hash for ParticleMaterial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            ParticleMaterial::Simple { color } => state.write_u64(color_as_u64(color)),
        }
    }
}

fn color_as_u64(color: &Color) -> u64 {
    match color.as_rgba() {
        Color::Rgba {
            red,
            green,
            blue,
            alpha,
        } => {
            let c = |c: f32| (c.clamp(0., 255.) * 255.) as u64;
            let x = 65536;
            c(red) + c(green) * x + c(blue) * x * x + c(alpha) * x * x * x
        }
        _ => unimplemented!(),
    }
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaterialCache>()
            .add_systems(Last, fix_lights);
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
