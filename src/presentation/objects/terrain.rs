use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
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
        let scene = assets.model_wall.clone();

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
        let scene = assets.model_floor.clone();

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
) {
    for (entity, object) in new.iter() {
        match object {
            TerrainLight::Generic => {
                commands.try_with_children(entity, |parent| {
                    parent.spawn(PointLightBundle {
                        point_light: PointLight {
                            color: Color::WHITE,
                            intensity: 3000.,
                            shadows_enabled: true,
                            ..default()
                        },
                        transform: Transform::from_xyz(0., -0.5, -2.5),
                        ..default()
                    });
                });
            }
        }
    }
}
