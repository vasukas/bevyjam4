use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::objects::enemy::Enemy;
use crate::gameplay::utils::InterpolateTransformOnce;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (spawn.in_set(SpawnSet::Controllers), on_death));
    }
}

/// Player graphical state
#[derive(Component)]
struct EnemyData {
    model: Entity,
}

fn spawn(new: Query<Entity, Added<Enemy>>, mut commands: Commands, assets: Res<ObjectAssets>) {
    for entity in new.iter() {
        let model = &assets.model_tripod;
        let scene = model.scene();

        commands.try_command(entity, |entity| {
            let model = entity.with_child(|parent| {
                parent
                    .spawn((SceneBundle {
                        scene,
                        transform: rotate_3to2_tr(),
                        ..default()
                    },))
                    .id()
            });
            entity.insert(EnemyData { model });
        });
    }
}

fn on_death(died: Query<&EnemyData, (With<Enemy>, Added<Dead>)>, mut commands: Commands) {
    let duration = Duration::from_millis(500);

    for data in died.iter() {
        commands.try_insert(
            data.model,
            InterpolateTransformOnce::new(duration)
                .rotation(default())
                .pos(Vec3::Z * 0.5),
        );
    }
}
