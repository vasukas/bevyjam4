use crate::app::scheduling::SpawnSet;
use crate::gameplay::mechanics::movement::MovementController;
use crate::gameplay::physics::TypicalBody;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_player.in_set(SpawnSet::Roots));
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
