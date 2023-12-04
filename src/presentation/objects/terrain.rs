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
    for (entity, _object) in new.iter() {
        let scene = assets.scene_wall.clone();

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
    for (entity, _object) in new.iter() {
        let scene = assets.scene_floor.clone();

        commands.try_with_children(entity, |parent| {
            parent.spawn(SceneBundle {
                scene,
                transform: rotate_3to2_tr(),
                ..default()
            });
        });
    }
}

fn spawn_terrain_light(
    new: Query<(Entity, &TerrainLight), Added<TerrainLight>>,
    mut commands: Commands,
    settings: Res<AppSettings>,
) {
    for (entity, object) in new.iter() {
        let shadows_enabled = settings.graphics.shadows;

        // up and away from wall
        let transform = Transform::from_xyz(0., -0.5, 2.5);

        match *object {
            TerrainLight::Custom {
                color,
                intensity,
                shadows,
            } => {
                commands.try_with_children(entity, move |parent| {
                    let shadows_enabled = shadows && shadows_enabled;
                    let mut child = parent.spawn(PointLightBundle {
                        point_light: PointLight {
                            color,
                            intensity,
                            shadows_enabled,
                            ..default()
                        },
                        transform,
                        ..default()
                    });
                    if shadows_enabled {
                        child.insert(LightWithShadows);
                    }
                });
            }

            TerrainLight::Generic => {
                commands.try_with_children(entity, move |parent| {
                    parent.spawn((
                        PointLightBundle {
                            point_light: PointLight {
                                color: Color::ALICE_BLUE,
                                intensity: 200.,
                                shadows_enabled,
                                ..default()
                            },
                            transform,
                            ..default()
                        },
                        LightWithShadows,
                    ));
                });
            }
        }
    }
}
