use super::animation_ctl::AnimationCtl;
use super::animation_ctl::AnimationCtlSystem;
use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::objects::conveyor::Conveyor;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_conv.in_set(SpawnSet::Controllers),
                update_conv_animation.before(AnimationCtlSystem),
            ),
        );
    }
}

#[derive(Component)]
struct ConvData {
    model: Entity,
}

fn spawn_conv(
    new: Query<(Entity, &Conveyor), Added<Conveyor>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        let (model,) = match object {
            Conveyor::Belt => (&assets.model_belt,),
            Conveyor::StartChute | Conveyor::EndChute => (&assets.model_chute,),
        };
        let scene = model.scene();
        // TODO: ANIMATIONS

        commands.try_command(entity, |entity| {
            let id = entity.with_child(|parent| {
                parent
                    .spawn((
                        SceneBundle {
                            scene,
                            transform: rotate_3to2_tr(),
                            ..default()
                        },
                        // animation,
                    ))
                    .id()
            });

            entity.insert(ConvData { model: id });
        });
    }
}

fn update_conv_animation(
    mut conveyors: Query<(&Conveyor, &mut ConvData)>,
    mut animations: Query<&mut AnimationCtl>,
    time: Res<Time>,
) {
    for (conv, mut data) in conveyors.iter_mut() {
        //
    }
}
