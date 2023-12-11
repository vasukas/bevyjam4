use crate::app::scheduling::SpawnSet;
use crate::gameplay::balance::OVERLOAD_ENEMY_REGULAR;
use crate::gameplay::master::level_progress::ImportantEnemy;
use crate::gameplay::mechanics::ai::*;
use crate::gameplay::mechanics::damage::DamageType;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::mechanics::overload::Overload;
use crate::gameplay::physics::*;
use crate::gameplay::utils::RotateToTarget;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::random::RandomVec;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub enum Enemy {
    Important,
    #[allow(unused)]
    Spam,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, spawn_player.in_set(SpawnSet::Roots));
    }
}

const ENEMY_RADIUS: f32 = 0.7;

fn spawn_player(
    mut new: Query<(Entity, &mut Transform, &Enemy), Added<Enemy>>,
    mut commands: Commands,
) {
    for (entity, mut transform, enemy) in new.iter_mut() {
        let target_dir = Vec2::random_dir();
        transform.rotation = Quat::from_rotation_z(-target_dir.angle_between(Vec2::X));

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
                RotateToTarget::new_from_time(0.5).with_target(target_dir),
                //
                Target::default(),
                Shoot {
                    period: Duration::from_millis(80),
                    projectile: Projectile {
                        damage: 1,
                        speed: 6.,
                        radius: 0.15,
                        ty: DamageType::Player,
                    },
                },
                //
                Overload::new(OVERLOAD_ENEMY_REGULAR),
            ),
        );

        match enemy {
            Enemy::Important => commands.try_insert(entity, ImportantEnemy),
            Enemy::Spam => (),
        }
    }
}
