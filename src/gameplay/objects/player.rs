use super::elevators::Elevator;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::*;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use bevy_rapier2d::geometry::CollidingEntities;

#[derive(Component, Debug, Default)]
pub struct Player {
    pub state: PlayerState,
    pub input_walking: bool,
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
                (update_player_state, on_collisions).after(MechanicSet::Reaction),
            );
    }
}

const PLAYER_RADIUS: f32 = 0.2;
pub const PLAYER_HEALTH: u32 = 30;

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
                Health::new(PLAYER_HEALTH),
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
