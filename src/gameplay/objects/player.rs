use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::TypicalBody;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_player.in_set(SpawnSet::Roots))
            .add_systems(Update, update_player_state.after(MechanicSet::Reaction));
    }
}

const PLAYER_RADIUS: f32 = 0.3;

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
                MovementController {
                    speed: 6.,
                    k_force: 100.,
                    ..default()
                }
                .bundle(),
                RotateToTarget::new_from_time(0.35),
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
