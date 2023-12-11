use super::player::Player;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::physics::*;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Conveyor {
    Belt,
    StartChute,
    EndChute,
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_conveyor.in_set(SpawnSet::Roots),
                update_conveyor
                    .after(PhysicsSet::Writeback)
                    .before(TransformSystem::TransformPropagate),
            ),
        );
    }
}

fn spawn_conveyor(new: Query<(Entity, &Conveyor), Added<Conveyor>>, mut commands: Commands) {
    for (entity, object) in new.iter() {
        //
    }
}

fn update_conveyor(
    convs: Query<(Entity, &mut Conveyor)>,
    colliding: Query<&CollidingEntities>,
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Transform, Has<Player>, Has<Dead>)>,
) {
    //
}
