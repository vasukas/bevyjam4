use super::elevators::Elevator;
use super::particles::Particle;
use super::particles::spawn_particle;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::DamageType;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::*;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy::misc_utils::ExtendedTimer;
use crate::utils::math_algorithms::dir_vec2;
use crate::utils::math_algorithms::lerp;
use crate::utils::math_algorithms::rotate_vec2;
use crate::utils::random::RandomRange;
use crate::utils::random::RandomVec;
use bevy::prelude::*;
use bevy_rapier2d::geometry::CollidingEntities;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component, Default)]
pub struct Player {
    pub state: PlayerState,
    pub input_walking: bool,

    pub input_kick: bool,
    pub input_locked: Timer,
    pub kick_animation: bool,
    kick_cooldown: Timer,
    kick_hit: Option<Timer>,

    pub input_fire: bool,
    pub input_pull: bool,

    fire_cooldown: Timer,
    fire_count: usize,

    pull_active: bool,
    pull_cooldown: Option<Timer>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
}

#[derive(Event)]
pub enum PlayerEvent {
    /// Sent each frame
    ReachedExitElevator,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEvent>()
            .add_systems(PostUpdate, spawn_player.in_set(SpawnSet::Roots))
            .add_systems(
                Update,
                (
                    (update_player_state, on_collisions).after(MechanicSet::Reaction),
                    (fire_input, update_pull, kick).in_set(MechanicSet::Action),
                ),
            );
    }
}

pub const PLAYER_HEALTH: u32 = 30;
const PLAYER_RADIUS: f32 = 0.2;

fn spawn_player(new: Query<Entity, Added<Player>>, mut commands: Commands) {
    for entity in new.iter() {
        commands.try_insert(
            entity,
            (
                TypicalBody::new_ball(PLAYER_RADIUS)
                    .friction(0.3)
                    .restitution(0.)
                    .mass(40.)
                    .lock_rotation(),
                PhysicsType::Object.groups(),
                MovementController {
                    speed: 6.,
                    k_force: 100.,
                    ..default()
                }
                .bundle(),
                RotateToTarget::new_from_time(0.35),
                Health {
                    value: PLAYER_HEALTH,
                    ty: DamageType::Player,
                },
                //
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
            ),
        );
    }
}

fn update_player_state(mut player: Query<&mut Player>, time: Res<Time>) {
    for mut player in player.iter_mut() {
        player.input_locked.tick(time.delta());

        if std::mem::take(&mut player.input_walking) {
            player.state = PlayerState::Walking;
        } else {
            player.state = PlayerState::Idle;
        }
    }
}

fn on_collisions(
    player: Query<&CollidingEntities, With<Player>>,
    elevators: Query<&Elevator>,
    mut events: EventWriter<PlayerEvent>,
) {
    for entities in player.iter() {
        for entity in entities.iter() {
            if elevators.get(entity) == Ok(&Elevator::Exit) {
                events.send(PlayerEvent::ReachedExitElevator)
            }
        }
    }
}

fn fire_input(
    mut player: Query<(&GlobalTransform, &mut Player)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (pos, mut player) in player.iter_mut() {
        let pos = pos.translation().truncate();

        player.fire_cooldown.tick(time.delta());

        if std::mem::take(&mut player.input_fire) && player.fire_cooldown.finished() {
            player.fire_cooldown = Timer::once(Duration::from_millis(300));

            let offset = (player.fire_count as f32 / 40.) % 1. * TAU;
            player.fire_count += 1;

            let count = 8;
            for index in 0..count {
                let da = TAU / count as f32;
                let angle = index as f32 * da + offset + (-da..da).random();

                let direction = dir_vec2(angle);
                let radius = 0.5;

                commands.spawn(
                    Projectile {
                        damage: 1,
                        speed: 8.,
                        radius,
                        ty: DamageType::Barrels,
                    }
                    .bundle(pos + direction * (PLAYER_RADIUS + radius + 0.5), direction),
                );

                spawn_particle(&mut commands, pos + direction * (PLAYER_RADIUS + radius + 0.5), Particle::FireImpact);
            }
        }
    }
}

fn update_pull(
    mut player: Query<(&GlobalTransform, &mut Player)>,
    time: Res<Time>,
    mut commands: Commands,
    objects: Query<&GlobalTransform, (Without<Player>, With<Collider>)>,
    physics: Res<RapierContext>,
) {
    let max_distance = 6_f32;
    let scale_in = 0.35;
    let scale_out = 0.6;
    let impulse = 500.;
    let min_time = Duration::from_millis(400);

    for (pos, mut player) in player.iter_mut() {
        // check if input is active
        if let Some(timer) = player.pull_cooldown.as_mut() {
            if timer.tick(time.delta()).finished() {
                player.pull_cooldown = None;
            }
        }
        let input_pull = std::mem::take(&mut player.input_pull) || player.pull_cooldown.is_some();

        // decide what to do
        let scale = if player.pull_active && !input_pull {
            // disable pull (once)

            player.pull_active = false;
            player.pull_cooldown = None;

            Some(scale_out)
        } else if input_pull {
            // apply pull (continiuosly)

            if !player.pull_active {
                player.pull_cooldown = Some(Timer::once(min_time));
            }
            player.pull_active = true;

            Some(-scale_in * time.delta_seconds())
        } else {
            None
        };

        // apply effect
        if let Some(scale) = scale {
            let pos = pos.translation().truncate();
            let radius = Collider::ball(max_distance - 0.5);

            let mut callback = |entity| {
                let Ok(transform) = objects.get(entity) else { return; };

                let target = transform.translation().truncate();
                let delta = target - pos;

                // don't go through walls
                if physics
                    .cast_ray(pos, delta, 1., false, PhysicsType::WallOnly.filter())
                    .is_some()
                {
                    return;
                }

                let distance = delta.length();
                let pull_closer = scale < 0.;

                let power = match pull_closer {
                    true => {
                        // reduce if close
                        lerp(0., 1., distance / (max_distance - 0.5))
                    }
                    false => 1.,
                };

                let dir = (delta + Vec2::random_dir() * 0.5).normalize_or_zero();
                let impulse = dir * scale * impulse * power;

                commands.try_insert(
                    entity,
                    ExternalImpulse {
                        impulse,
                        ..default()
                    },
                )
            };

            physics.intersections_with_shape(
                pos,
                0.,
                &radius,
                PhysicsType::GravityPull.filter(),
                |entity| {
                    callback(entity);
                    true
                },
            );
        }
    }
}

fn kick(
    mut player: Query<(&GlobalTransform, &mut Player)>,
    objects: Query<&GlobalTransform, (Without<Player>, With<Collider>)>,
    physics: Res<RapierContext>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let cooldown = Duration::from_millis(500);
    let hit_time = Duration::from_secs_f32(0.7 * 0.33); // sync to animation
    let impulse = 800.;
    let distance = 0.8;
    let width = PLAYER_RADIUS * 2.;

    for (transform, mut player) in player.iter_mut() {
        player.kick_cooldown.tick(time.delta());

        if std::mem::take(&mut player.input_kick) && player.kick_cooldown.finished() {
            // player.input_locked = Timer::once(cooldown);
            player.kick_cooldown = Timer::once(cooldown);
            player.kick_hit = Some(Timer::once(hit_time));
            player.kick_animation = true;
        }

        if let Some(timer) = player.kick_hit.as_mut() {
            if timer.tick(time.delta()).finished() {
                player.kick_hit = None;

                let transform = Transform::from(*transform);
                let pos = transform.translation.truncate();

                let mut callback = |entity| {
                    let Ok(transform) = objects.get(entity) else { return; };

                    let target = transform.translation().truncate();
                    let delta = target - pos;

                    let dir = delta.normalize_or_zero();
                    let impulse = dir * impulse;

                    commands.try_insert(
                        entity,
                        ExternalImpulse {
                            impulse,
                            ..default()
                        },
                    )
                };

                let angle = transform.rotation.to_euler(EulerRot::ZYX).0;
                let forward = rotate_vec2(Vec2::X, angle);

                physics.intersections_with_shape(
                    pos + forward * (distance / 2.),
                    angle,
                    &Collider::cuboid(distance / 2., width / 2.),
                    PhysicsType::GravityPull.filter(),
                    |entity| {
                        callback(entity);
                        true
                    },
                );
            }
        }
    }
}
