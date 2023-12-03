use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level::data::HALF_TILE;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component, Clone, Serialize, Deserialize, Debug, Default)]
pub enum TerrainWall {
    #[default]
    #[serde(other)]
    Generic,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, Default)]
pub enum TerrainFloor {
    #[default]
    #[serde(other)]
    Generic,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, Default)]
pub enum TerrainLight {
    #[default]
    #[serde(other)]
    Generic,
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_terrain_colliders.in_set(SpawnSet::Roots));
    }
}

const WALL_THICKNESS: f32 = 0.2;

fn spawn_terrain_colliders(walls: Query<Entity, Added<TerrainWall>>, mut commands: Commands) {
    for entity in walls.iter() {
        commands.try_insert(
            entity,
            (
                RigidBody::Fixed,
                Collider::cuboid(HALF_TILE, WALL_THICKNESS / 2.),
            ),
        );
    }
}
