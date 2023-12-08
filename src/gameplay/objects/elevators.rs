use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level::data::HALF_TILE;
use crate::gameplay::physics::*;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component, Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub enum Elevator {
    #[default]
    Enter,
    Exit,
}

pub struct ElevatorsPlugin;

impl Plugin for ElevatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_elevator.in_set(SpawnSet::Roots));
    }
}

fn spawn_elevator(new: Query<Entity, Added<Elevator>>, mut commands: Commands) {
    for entity in new.iter() {
        commands.try_insert(
            entity,
            (
                TypicalBody {
                    body: RigidBody::Fixed,
                    ..TypicalBody::new_box(Vec2::splat(HALF_TILE))
                },
                PhysicsType::Wall.groups(),
            ),
        );
    }
}
