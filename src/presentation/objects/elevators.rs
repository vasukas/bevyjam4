use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level_progress::ExitUnlocked;
use crate::gameplay::objects::elevators::Elevator;
use crate::gameplay::objects::terrain::TerrainLight;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct ElevatorsPlugin;

impl Plugin for ElevatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_elevators,
                unlock_exit_elevator.run_if(on_event::<ExitUnlocked>()),
            )
                .in_set(SpawnSet::Controllers),
        );
    }
}

fn spawn_elevators(
    new: Query<(Entity, &Elevator), Added<Elevator>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, object) in new.iter() {
        let bundle = SceneBundle {
            scene: match object {
                Elevator::Enter => assets.scene_elevator_enter.clone(),
                Elevator::Exit => assets.scene_elevator_exit.clone(),
            },
            transform: rotate_3to2_tr(),
            ..default()
        };

        let bundle2 = elevator_lamp_bundle(object, false);

        commands.try_command(entity, |entity| {
            let id = entity.with_child(|parent| {
                parent.spawn(bundle);
                parent.spawn(bundle2).id()
            });
            entity.insert(ElevatorLamp(id));
        });
    }
}

#[derive(Component)]
struct ElevatorLamp(Entity);

fn unlock_exit_elevator(
    elevators: Query<(Entity, &Elevator, &ElevatorLamp)>,
    mut commands: Commands,
    mut triggered: EventReader<ExitUnlocked>,
) {
    let _ = triggered.read(); // TODO: maybe this will fix the bug?

    for (entity, object, lamp) in elevators.iter() {
        if object == &Elevator::Exit {
            commands.try_despawn_recursive(lamp.0);

            let bundle2 = elevator_lamp_bundle(object, true);
            commands.try_with_children(entity, |parent| {
                parent.spawn(bundle2);
            });
        }
    }
}

fn elevator_lamp_bundle(elevator: &Elevator, unlocked: bool) -> impl Bundle {
    (
        SpatialBundle::default(),
        TerrainLight::Custom {
            color: match (elevator, unlocked) {
                (Elevator::Enter, _) => Color::BLUE * 1.,
                (Elevator::Exit, false) => Color::RED * 2.,
                (Elevator::Exit, true) => Color::GREEN * 3.,
            },
            intensity: 50.,
            shadows: false,
        },
    )
}
