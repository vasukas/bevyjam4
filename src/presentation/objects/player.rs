use super::animation_ctl::AnimationCtl;
use super::animation_ctl::AnimationCtlSystem;
use super::assets::ModelAsset;
use super::assets::ObjectAssets;
use super::utils::rotate_3to2_tr;
use super::WorldCameraBundle;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::objects::player::Player;
use crate::gameplay::objects::player::PlayerState;
use crate::utils::bevy::commands::ExtendedEntityMut;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy::misc_utils::ImmediateTransformUpdate;
use crate::utils::misc_utils::ExtendedTime;
use crate::utils::random::RandomBool as _;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_player.in_set(SpawnSet::Controllers),
                camera_tracking.after(TransformSystem::TransformPropagate),
                update_player_animation.before(AnimationCtlSystem),
            ),
        );
    }
}

/// Marker for camera which follows player
#[derive(Component)]
struct PlayerCamera;

/// Player graphical state
#[derive(Component)]
struct PlayerData {
    model: Entity,
    idle_for: Duration,
    had_health: u32,
}

/// In order of increasing priority
#[derive(Clone, Copy, PartialEq, Eq)]
enum PlayerAnimation {
    Idle,
    LookAround,
    LookBack,
    Walking,
    Hit,
    Kick,
    Dead,
    Death,
    // don't forget to add new ones to all()
}

impl Into<usize> for PlayerAnimation {
    fn into(self) -> usize {
        self as usize
    }
}

impl PlayerAnimation {
    fn add(self, animation: &mut AnimationCtl, model: &ModelAsset) {
        let (name, repeat, duration) = match self {
            Self::Idle => ("idle", true, 1.),
            Self::LookAround => ("look_around", false, 3.),
            Self::LookBack => ("look_back", false, 3.),
            Self::Walking => ("walk", true, 0.8),
            Self::Hit => ("hit", false, 0.4),
            Self::Kick => ("kick", false, 0.7),
            Self::Dead => ("dead", true, 1.),
            Self::Death => ("death", false, 1.),
        };
        animation.layer(self, model.animation(name), repeat, 1. / duration)
    }

    fn all() -> impl Iterator<Item = Self> {
        [
            Self::Idle,
            Self::LookAround,
            Self::LookBack,
            Self::Walking,
            Self::Hit,
            Self::Kick,
            Self::Dead,
            Self::Death,
        ]
        .into_iter()
    }

    fn make_ctl(model: &ModelAsset) -> AnimationCtl {
        let mut ctl = AnimationCtl::default();
        Self::all().for_each(|a| a.add(&mut ctl, model));
        ctl
    }
}

fn spawn_player(
    new: Query<Entity, Added<Player>>,
    mut commands: Commands,
    assets: Res<ObjectAssets>,
) {
    for entity in new.iter() {
        let model = &assets.model_jimbo;
        let scene = model.scene();
        let animation = PlayerAnimation::make_ctl(&model);

        commands.try_command(entity, |entity| {
            let id = entity.with_child(|parent| {
                parent
                    .spawn((
                        SceneBundle {
                            scene,
                            transform: rotate_3to2_tr(),
                            ..default()
                        },
                        animation,
                    ))
                    .id()
            });

            entity.insert(PlayerData {
                model: id,
                idle_for: default(),
                had_health: default(),
            });
        });

        commands.spawn((WorldCameraBundle::new("player camera"), PlayerCamera));

        // TODO: set per-level?
        commands.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.15,
        });
    }
}

fn camera_tracking(
    camera: Query<Entity, With<PlayerCamera>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut transform: ImmediateTransformUpdate<Without<Player>>,
) {
    let Some((camera, player)) = camera.get_single().ok().zip(player.get_single().ok()) else { return; };

    transform.update_inplace(camera, |transform| {
        let z = transform.translation.z;
        transform.translation = player.translation();
        transform.translation.z = z;
    });
}

fn update_player_animation(
    mut player: Query<(&mut Player, &mut PlayerData, &Health)>,
    mut animations: Query<&mut AnimationCtl>,
    time: Res<Time>,
    died: Query<(), (With<Player>, Added<Dead>)>,
) {
    let idle_after = Duration::from_secs(1);
    let idle_check_period = Duration::from_secs(3);
    let idle_chance = 0.8;
    let idle_chance_around = 0.6;

    for (mut player, mut data, health) in player.iter_mut() {
        let Ok(mut animation) = animations.get_mut(data.model) else { continue; };

        // Hit animation is disabled - it looks janky and can't be seen through particles anyway
        // if health.value < data.had_health {
        //     animation.set_active(PlayerAnimation::Hit, true);
        // }
        data.had_health = health.value;

        if !died.is_empty() {
            animation.set_active(PlayerAnimation::Dead, true);
            animation.set_active(PlayerAnimation::Death, true);
        }

        if std::mem::take(&mut player.kick_animation) {
            animation.set_active(PlayerAnimation::Kick, true);
        }

        // random idle animations
        if player.state == PlayerState::Idle {
            data.idle_for += time.delta();

            if data.idle_for > idle_after
                && time.is_tick(idle_check_period, default())
                && bool::true_with_chance(idle_chance)
            {
                let anim = match bool::true_with_chance(idle_chance_around) {
                    true => PlayerAnimation::LookAround,
                    false => PlayerAnimation::LookBack,
                };
                animation.set_active(anim, true);
            }
        } else {
            data.idle_for = default();
        }

        animation.set_active(
            PlayerAnimation::Walking,
            player.state == PlayerState::Walking,
        );
    }
}
