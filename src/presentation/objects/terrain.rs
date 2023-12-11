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
            (
                spawn_terrain_wall,
                spawn_terrain_decor,
                spawn_terrain_floor,
                spawn_terrain_light,
                spawn_unique,
            )
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
            TerrainWall::WallCharred1 => assets.scene_wall_charred1.clone(),
            TerrainWall::WallCharred2 => assets.scene_wall_charred2.clone(),
            TerrainWall::CellBars => assets.scene_cell_bars.clone(),
            TerrainWall::Computer => assets.scene_wall_computer.clone(),
            TerrainWall::ComputerScreen => assets.scene_wall_compscreen.clone(),
            TerrainWall::Hatch => assets.scene_wall_hatch.clone(),
            TerrainWall::Ventilation => assets.scene_wall_ventilation.clone(),
            TerrainWall::VerticalPipe => assets.scene_wall_vertpipe.clone(),
            TerrainWall::VerticalPipe2 => assets.scene_wall_vertpipe2.clone(),
            TerrainWall::HorizontalPipes => assets.scene_wall_horpipes.clone(),
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

fn spawn_terrain_decor(
    new: Query<(Entity, &TerrainDecor), Added<TerrainDecor>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        let scene = match object {
            TerrainDecor::CellBed => assets.scene_cell_bed.clone(),
            TerrainDecor::LoadCrane => assets.scene_load_crane.clone(),
            TerrainDecor::ClosedPipe => assets.scene_closed_pipe.clone(),
            TerrainDecor::GreenPipe => assets.scene_green_pipe.clone(),
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
            TerrainFloor::CellMelted => (assets.scene_cell_melted.clone(), 0.),
            TerrainFloor::Hatch => (assets.scene_floor_hatch.clone(), 0.),
            TerrainFloor::Metals => (assets.scene_floor_metals.clone(), 0.),

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
            TerrainLight::Alarm => (Color::RED, 60., true),
            TerrainLight::AlarmBright => (Color::rgb(1., 0.5, 0.4), 200., true),
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

fn spawn_unique(
    new: Query<(Entity, &UniqueDecor), Added<UniqueDecor>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        let scene = match object {
            UniqueDecor::EngineFurnace => assets.scene_engine.clone(),
            UniqueDecor::MegaBrain => assets.scene_brain.clone(),
            UniqueDecor::Cannon => assets.scene_cannon.clone(),
        };
        let brainlights = matches!(object, UniqueDecor::MegaBrain);
        let enginelights = matches!(object, UniqueDecor::EngineFurnace);

        commands.try_with_children(entity, move |parent| {
            parent.spawn(SceneBundle {
                scene,
                transform: rotate_3to2_tr(),
                ..default()
            });

            if brainlights {
                parent.spawn(PointLightBundle {
                    point_light: PointLight {
                        intensity: 100.,
                        shadows_enabled: false,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..default()
                });

                parent.spawn(PointLightBundle {
                    point_light: PointLight {
                        intensity: 500.,
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., 5.),
                    ..default()
                });
            }

            if enginelights {
                parent.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::ORANGE_RED,
                        intensity: 2000.,
                        shadows_enabled: false,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., 0.5),
                    ..default()
                });
            }
        });
    }
}
