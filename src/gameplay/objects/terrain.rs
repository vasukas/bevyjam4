use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level::data::HALF_TILE;
use crate::gameplay::physics::PhysicsType;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component, Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum TerrainWall {
    Computer,
    ComputerScreen,
    Hatch,
    Ventilation,
    VerticalPipe,
    VerticalPipe2,
    HorizontalPipes,
    CellBars,

    #[default]
    #[serde(other)]
    Generic,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum TerrainFloor {
    VoidLta,
    VoidLtb,
    VoidSquare,
    VoidTriangle,
    CellMelted,

    Hatch,
    Metals,

    #[default]
    #[serde(other)]
    Generic,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum TerrainLight {
    Custom {
        color: Color,
        intensity: f32,
        shadows: bool,
    },
    Alarm,
    AlarmBright,

    #[default]
    #[serde(other)]
    Generic,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum TerrainDecor {
    CellBed,
    LoadCrane,

    ClosedPipe,
    GreenPipe,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum UniqueDecor {
    EngineFurnace,
    MegaBrain,
    Cannon,
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_terrain_colliders.in_set(SpawnSet::Roots));
    }
}

const WALL_THICKNESS: f32 = 0.2;

fn spawn_terrain_colliders(
    walls: Query<Entity, Added<TerrainWall>>,
    decors: Query<Entity, Added<TerrainDecor>>,
    uniques: Query<Entity, Added<UniqueDecor>>,
    mut commands: Commands,
) {
    for entity in walls.iter() {
        commands.try_insert(
            entity,
            (
                RigidBody::Fixed,
                Collider::cuboid(HALF_TILE, WALL_THICKNESS / 2.),
                PhysicsType::Wall.groups(),
            ),
        );
    }

    for entity in decors.iter() {
        commands.try_insert(
            entity,
            (
                RigidBody::Fixed,
                Collider::cuboid(HALF_TILE, HALF_TILE),
                PhysicsType::Wall.groups(),
            ),
        );
    }

    for entity in uniques.iter() {
        commands.try_insert(
            entity,
            (
                RigidBody::Fixed,
                Collider::cuboid(2., 2.), // 2x2 tiles
                PhysicsType::Wall.groups(),
            ),
        );
    }
}
