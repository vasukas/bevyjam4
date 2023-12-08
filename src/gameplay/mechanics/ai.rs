use super::damage::Dead;
use super::damage::Projectile;
use super::MechanicSet;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level::data::TILE_SIZE;
use crate::gameplay::objects::player::Player;
use crate::gameplay::physics::PhysicsType;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::math_algorithms::map_linear_range;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

/// Find the target
#[derive(Component, Default)]
pub struct Target {
    /// Has line-of-sight to target
    found: Option<TargetData>,

    visible_for: Duration,
    invisible_for: Duration,
}

impl Target {
    /// Max visibility distance
    const MAX_DISTANCE: f32 = TILE_SIZE * 8.;

    /// AI has delayed reaction to player appearing in their field of view
    const MIN_REACTION_DELAY: Duration = Duration::from_millis(1000);
    const MAX_REACTION_DELAY: Duration = Duration::from_millis(2000);

    /// Time in which AI forgets it has seen the player
    const FORGET_TIME: Duration = Duration::from_millis(2000);

    /// Can react to player - delay time has passed
    fn can_react(&self) -> bool {
        let Some(data) = self.found else { return false; };

        let time = map_linear_range(
            data.distance,
            0.,
            Self::MAX_DISTANCE,
            Self::MIN_REACTION_DELAY.as_secs_f32(),
            Self::MAX_REACTION_DELAY.as_secs_f32(),
            true,
        );

        self.visible_for >= Duration::from_secs_f32(time)
    }
}

#[derive(Clone, Copy)]
struct TargetData {
    dir: Vec2, // direction, normalized
    distance: f32,
}

/// Shoot the target
#[derive(Component)]
pub struct Shoot {
    pub period: Duration,
    pub projectile: Projectile,
}

#[derive(Component, Default)]
struct ShootState {
    cooldown: Timer,
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn.in_set(SpawnSet::Controllers))
            .add_systems(
                Update,
                (find_target, rotate, shoot)
                    .chain()
                    .in_set(MechanicSet::Action),
            );
    }
}

fn spawn(new_shoot: Query<Entity, Added<Shoot>>, mut commands: Commands) {
    for entity in new_shoot.iter() {
        commands.try_insert(entity, ShootState::default());
    }
}

fn find_target(
    target: Query<&GlobalTransform, With<Player>>,
    mut finders: Query<(&GlobalTransform, &mut Target), Without<Dead>>,
    phy_world: Res<RapierContext>,
    time: Res<Time>,
) {
    let target_pos = target
        .get_single()
        .map(|pos| pos.translation().truncate())
        .ok();

    for (pos, mut target) in finders.iter_mut() {
        target.found = target_pos.map(|target| {
            let pos = pos.translation().truncate();

            let dir = (target - pos).try_normalize().unwrap_or(Vec2::Y);
            let distance = (target - pos).length();

            TargetData { dir, distance }
        });

        // // update target data
        // target.found = target_pos.and_then(|target| {
        //     let pos = pos.translation().truncate();

        //     let dir = (target - pos).try_normalize().unwrap_or(Vec2::Y);
        //     let distance = (target - pos).length();

        //     // visible if no opaque object is blocking ray
        //     let visible = distance < Target::MAX_DISTANCE
        //         && phy_world
        //             .cast_ray(
        //                 pos,
        //                 dir,
        //                 Target::MAX_DISTANCE,
        //                 true,
        //                 PhysicsType::WallOnly.filter(),
        //             )
        //             .is_none();

        //     visible.then_some(TargetData { dir, distance })
        // });

        // reaction delay / forget
        let visible = target.found.is_some();
        if visible {
            target.visible_for += time.delta();
            target.invisible_for = default();
        } else {
            target.invisible_for += time.delta();
            if target.invisible_for >= Target::FORGET_TIME {
                target.visible_for = default();
            }
        }
    }
}

fn rotate(mut entities: Query<(&mut RotateToTarget, &Target), Without<Dead>>) {
    for (mut rotate, target) in entities.iter_mut() {
        let Some(target) = target.found else { continue; };
        rotate.target_dir = target.dir;
    }
}

fn shoot(
    mut shooters: Query<
        (Entity, &Shoot, &mut ShootState, &Target, &GlobalTransform),
        Without<Dead>,
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, shoot, mut state, target, pos) in shooters.iter_mut() {
        let Some(target) = target.found.filter(|_| target.can_react()) else { continue; };

        state.cooldown.tick(time.delta());

        for _ in 0..state.cooldown.times_finished_this_tick() {
            let cooldown = shoot.period;
            state.cooldown = Timer::new(cooldown, TimerMode::Repeating);

            commands.spawn(
                shoot
                    .projectile
                    .bundle(pos.translation().truncate(), target.dir),
            );
        }
    }
}
