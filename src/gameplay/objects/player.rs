use super::elevators::Elevator;
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
use crate::utils::random::RandomRange;
use bevy::prelude::*;
use bevy_rapier2d::geometry::CollidingEntities;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component, Default)]
pub struct Player {
    pub state: PlayerState,
    pub input_walking: bool,

    pub input_fire: bool,
    pub input_pull: bool,

    fire_cooldown: Timer,
    pull_cooldown: Timer,
    fire_count: usize,
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
                    weapon_input.in_set(MechanicSet::Action),
                    (update_player_state, on_collisions).after(MechanicSet::Reaction),
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

fn update_player_state(mut player: Query<&mut Player>) {
    for mut player in player.iter_mut() {
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

fn weapon_input(
    mut player: Query<(&GlobalTransform, &mut Player)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (pos, mut player) in player.iter_mut() {
        let pos = pos.translation().truncate();

        player.fire_cooldown.tick(time.delta());
        player.pull_cooldown.tick(time.delta());

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
            }
        }

        if std::mem::take(&mut player.input_pull) && player.pull_cooldown.finished() {
            //
        }
    }
}
