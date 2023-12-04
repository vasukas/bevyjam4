use super::assets::ObjectAssets;
use super::sprite::SimpleSprite;
use super::WorldCameraBundle;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::objects::player::Player;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy::misc_utils::ImmediateTransformUpdate;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_player.in_set(SpawnSet::Controllers),
                camera_tracking.after(TransformSystem::TransformPropagate),
            ),
        );
    }
}

/// Marker for camera which follows player
#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct ModelChild(Entity);

fn spawn_player(
    new: Query<Entity, Added<Player>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for entity in new.iter() {
        let scene = assets.model_jimbo.clone();

        // commands.try_command(entity, |entity| {
        //     let id =
        //         entity.with_child(|parent| parent.spawn(SceneBundle { scene, ..default() }).id());
        //     entity.insert(ModelChild(id));
        // });

        commands.try_insert(
            entity,
            SimpleSprite {
                color: Color::WHITE,
                size: Vec2::splat(0.5),
                z_offset: 10.,
                ..default()
            },
        );

        commands.spawn((WorldCameraBundle::new("player camera"), PlayerCamera));
    }
}

fn camera_tracking(
    camera: Query<Entity, With<PlayerCamera>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut transform: ImmediateTransformUpdate<Without<Player>>,
) {
    let Some((camera, player)) = camera.get_single().ok().zip(player.get_single().ok()) else { return; };

    transform.update_inplace(camera, |transform| {
        transform.translation = player.translation();
    });
}
