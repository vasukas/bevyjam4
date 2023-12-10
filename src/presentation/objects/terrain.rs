use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::app::settings::AppSettings;
use crate::app::settings::LightWithShadows;
use crate::gameplay::objects::terrain::*;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (spawn_terrain_wall, spawn_terrain_floor, spawn_terrain_light)
                .in_set(SpawnSet::Controllers),
        );
    }
}

fn spawn_terrain_wall(
    new: Query<(Entity, &TerrainWall), Added<TerrainWall>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        let scene = match object {
            TerrainWall::Generic => assets.scene_wall.clone(),
        };

        commands.try_with_children(entity, |parent| {
            parent.spawn(SceneBundle {
                scene,
                transform: rotate_3to2_tr(),
                ..default()
            });
        });
    }
}

fn spawn_terrain_floor(
    new: Query<(Entity, &TerrainFloor), Added<TerrainFloor>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    let void_offset = 2.95;

    for (entity, object) in new.iter() {
        let (scene, z_offset) = match object {
            TerrainFloor::Generic => (assets.scene_floor.clone(), 0.),

            TerrainFloor::VoidLta => (assets.scene_void_lta.clone(), void_offset),
            TerrainFloor::VoidLtb => (assets.scene_void_ltb.clone(), void_offset),
            TerrainFloor::VoidSquare => (assets.scene_void_squ.clone(), void_offset),
            TerrainFloor::VoidTriangle => (assets.scene_void_tri.clone(), void_offset),
        };

        commands.try_with_children(entity, move |parent| {
            parent.spawn(SceneBundle {
                scene,
                transform: rotate_3to2_tr().with_translation(Vec3::Z * z_offset),
                ..default()
            });
        });
    }
}

fn spawn_terrain_light(
    new: Query<(Entity, &TerrainLight), Added<TerrainLight>>,
    mut commands: Commands,
    settings: Res<AppSettings>,
    assets: Res<ObjectAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, object) in new.iter() {
        let (color, intensity, shadows) = match *object {
            TerrainLight::Custom {
                color,
                intensity,
                shadows,
            } => (color, intensity, shadows),

            TerrainLight::Generic => (Color::ALICE_BLUE, 200., true),
        };

        let light_transform = Transform::from_xyz(0., -0.5, 2.5); // up and away from wall
        let pbr_transform = Transform::from_xyz(0., -0.15, light_transform.translation.z)
            .with_scale(Vec3::splat(0.2));
        let pbr_intensity = 2.;

        let shadows_enabled = shadows && settings.graphics.shadows;

        let pbr = PbrBundle {
            mesh: assets.mesh_cube.clone(),
            material: materials.add(StandardMaterial {
                base_color: color * pbr_intensity,
                unlit: true,
                ..default()
            }),
            transform: pbr_transform,
            ..default()
        };

        commands.try_with_children(entity, move |parent| {
            let mut child = parent.spawn(PointLightBundle {
                point_light: PointLight {
                    color,
                    intensity,
                    shadows_enabled,
                    ..default()
                },
                transform: light_transform,
                ..default()
            });
            if shadows_enabled {
                child.insert(LightWithShadows);
            }
            // show light fixture for lamps
            if shadows {
                parent.spawn(pbr);
            }
        });
    }
}
