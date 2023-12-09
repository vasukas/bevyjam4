use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::objects::barrels::Barrel;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

pub struct BarrelsPlugin;

impl Plugin for BarrelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (spawn_barrels.in_set(SpawnSet::Controllers),));
    }
}

#[derive(Component)]
struct BarrelData {
    scene: Entity,
}

fn spawn_barrels(
    new: Query<(Entity, &Barrel), Added<Barrel>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for (entity, barrel) in new.iter() {
        let scene = match barrel {
            Barrel::Fire => assets.scene_barrel_red.clone(),
        };

        commands.try_command(entity, |entity| {
            let scene = entity.with_child(|parent| {
                parent
                    .spawn(SceneBundle {
                        scene,
                        transform: rotate_3to2_tr(),
                        ..default()
                    })
                    .id()
            });
            entity.insert(BarrelData { scene });
        });
    }
}
