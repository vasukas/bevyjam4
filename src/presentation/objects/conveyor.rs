use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::objects::conveyor::Conveyor;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (spawn_conv.in_set(SpawnSet::Controllers),));
    }
}

fn spawn_conv(
    new: Query<(Entity, &Conveyor), Added<Conveyor>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        match object {
            Conveyor::Belt => {
                let scene = assets.scene_belt.clone();

                commands.try_with_children(entity, |parent| {
                    parent.spawn(SceneBundle {
                        scene,
                        transform: rotate_3to2_tr(),
                        ..default()
                    });
                });
            }

            Conveyor::StartChute(..) | Conveyor::EndChute => {
                let scene = assets.scene_chute.clone();

                commands.try_with_children(entity, |parent| {
                    parent.spawn(SceneBundle {
                        scene,
                        transform: rotate_3to2_tr(),
                        ..default()
                    });
                });
            }
        }
    }
}
