use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::objects::conveyor::Conveyor;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use std::time::Duration;

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_conv.in_set(SpawnSet::Controllers),
                belt_animation
                    .after(SpawnSet::Details)
                    .before(TransformSystem::TransformPropagate),
            ),
        );
    }
}

const BELT_COUNT: usize = 4;

#[derive(Component)]
struct Belt {
    ids: [Entity; BELT_COUNT],
}

fn spawn_conv(
    new: Query<(Entity, &Conveyor), Added<Conveyor>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        match object {
            Conveyor::Belt => {
                let scenes = [
                    assets.scene_belt1.clone(),
                    assets.scene_belt2.clone(),
                    assets.scene_belt3.clone(),
                    assets.scene_belt4.clone(),
                ];
                commands.try_command(entity, |entity| {
                    let ids = scenes.map(|scene| {
                        entity.with_child(|parent| {
                            parent
                                .spawn(SceneBundle {
                                    scene,
                                    transform: rotate_3to2_tr(),
                                    ..default()
                                })
                                .id()
                        })
                    });
                    entity.insert(Belt { ids });
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

fn belt_animation(
    belts: Query<&Belt>,
    mut parts: Query<&mut Visibility>,
    time: Res<Time>,
    mut phase: Local<usize>,
    mut timer: Local<Option<Timer>>,
) {
    let period = Duration::from_millis(250);

    let timer = timer.get_or_insert_with(|| Timer::new(period, TimerMode::Repeating));

    let times = timer.tick(time.delta()).times_finished_this_tick();
    if times != 0 {
        *phase = (*phase + times as usize) % BELT_COUNT;

        for belt in belts.iter() {
            for (index, entity) in belt.ids.iter().enumerate() {
                if let Ok(mut part) = parts.get_mut(*entity) {
                    *part = match index == *phase {
                        true => Visibility::Visible,
                        false => Visibility::Hidden,
                    };
                }
            }
        }
    }
}
