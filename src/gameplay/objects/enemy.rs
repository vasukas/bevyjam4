use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level_progress::ImportantEnemy;
use crate::gameplay::mechanics::ai::*;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::physics::*;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_player.in_set(SpawnSet::Roots));
    }
}

const ENEMY_RADIUS: f32 = 0.7;

fn spawn_player(new: Query<Entity, Added<Enemy>>, mut commands: Commands) {
    for entity in new.iter() {
        commands.try_insert(
            entity,
            (
                TypicalBody {
                    body: RigidBody::Fixed,
                    ..TypicalBody::new_ball(ENEMY_RADIUS)
                        .friction(0.3)
                        .restitution(0.)
                        .mass(120.)
                },
                PhysicsType::Enemy.groups(),
                RotateToTarget::new_from_time(0.5),
                //
                Target::default(),
                Shoot {
                    period: Duration::from_millis(50),
                    projectile: Projectile::default(),
                },
                ImportantEnemy,
            ),
        );
    }
}
