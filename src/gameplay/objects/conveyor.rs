use super::barrels::Barrel;
use super::enemy::Enemy;
use super::player::Player;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level::data::HALF_TILE;
use crate::gameplay::master::level::data::TILE_SIZE;
use crate::gameplay::master::level::spawn::GameObjectBundle;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::physics::*;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::math_algorithms::rotate_vec2;
use crate::utils::random::RandomRange;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::utils::HashSet;
use serde::Deserialize;
use serde::Serialize;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Conveyor {
    Belt,
    StartChute(ConveyorOutput),
    EndChute,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum ConveyorOutput {
    #[default]
    BarrelsRareGroup,

    BarrelsFrequent,
    RandomEnemies,
    Random,
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                spawn_conveyor.in_set(SpawnSet::Roots),
                (belt_move, chute_spawn, chute_destroy)
                    .after(PhysicsSet::Writeback)
                    .before(TransformSystem::TransformPropagate),
            ),
        );
    }
}

#[derive(Component)]
struct Belt;

#[derive(Component, Default)]
struct ChuteSpawn {
    single_after: Duration,
    burst_after: Duration,
    burst_left: usize,
    output: ConveyorOutput,
}

#[derive(Component)]
struct ChuteDespawn;

fn spawn_conveyor(new: Query<(Entity, &Conveyor), Added<Conveyor>>, mut commands: Commands) {
    for (entity, object) in new.iter() {
        match *object {
            Conveyor::Belt => {
                commands.try_insert(
                    entity,
                    (
                        RigidBody::Fixed,
                        Collider::cuboid(HALF_TILE - 0.4, HALF_TILE),
                        Sensor,
                        //
                        PhysicsType::Conveyor.groups(),
                        CollidingEntities::default(),
                        ActiveEvents::COLLISION_EVENTS,
                        //
                        Belt,
                    ),
                );
            }

            Conveyor::StartChute(output) => {
                commands.try_insert(
                    entity,
                    ChuteSpawn {
                        output,
                        ..default()
                    },
                );
            }

            Conveyor::EndChute => {
                commands.try_with_child(
                    entity,
                    (
                        SpatialBundle::from_transform(Transform::from_translation(
                            Vec3::Y * TILE_SIZE,
                        )),
                        //
                        RigidBody::Fixed,
                        Collider::cuboid(HALF_TILE, HALF_TILE),
                        Sensor,
                        //
                        PhysicsType::Conveyor.groups(),
                        CollidingEntities::default(),
                        ActiveEvents::COLLISION_EVENTS,
                        //
                        ChuteDespawn,
                    ),
                );
            }
        }
    }
}

fn belt_move(
    colliding: Query<(&CollidingEntities, &Transform), With<Belt>>,
    mut entities: Query<&mut Transform, Without<Belt>>,
    time: Res<Time>,
) {
    let speed = 3. * time.delta_seconds();

    let mut affected: HashSet<_> = default();

    for (colliding, transform) in colliding.iter() {
        if !colliding.is_empty() {
            let angle = transform.rotation.to_euler(EulerRot::ZYX).0 - FRAC_PI_2;
            let forward = rotate_vec2(Vec2::X, angle) * speed;

            for entity in colliding.iter() {
                if let Ok(mut transform) = entities.get_mut(entity) {
                    if affected.insert(entity) {
                        transform.translation.x += forward.x;
                        transform.translation.y += forward.y;
                    }
                }
            }
        }
    }
}

fn chute_spawn(
    mut spawners: Query<(&Transform, &mut ChuteSpawn)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (transform, mut spawner) in spawners.iter_mut() {
        let spawner = &mut *spawner;

        let mut check = |burst_cooldown: Duration, single_delay: Duration, burst_count: usize| {
            let burst_count = burst_count.max(1);

            if time.elapsed() >= spawner.burst_after {
                spawner.burst_after =
                    time.elapsed() + burst_cooldown + single_delay * (burst_count - 1) as u32;
                spawner.burst_left = burst_count;
                spawner.single_after = time.elapsed();
            }

            if time.elapsed() >= spawner.single_after && spawner.burst_left != 0 {
                spawner.burst_left -= 1;
                if spawner.burst_left != 0 {
                    spawner.single_after = spawner.single_after + single_delay;
                }
                true
            } else {
                false
            }
        };

        let pos = || {
            let angle = transform.rotation.to_euler(EulerRot::ZYX).0 - FRAC_PI_2;
            let forward = rotate_vec2(Vec2::X, angle);

            let pos = transform.translation.truncate() - forward * TILE_SIZE;
            Transform::from_translation(pos.extend(0.))
        };

        match spawner.output {
            ConveyorOutput::BarrelsRareGroup => {
                if check(Duration::from_millis(6000), Duration::from_millis(400), 4) {
                    commands.spawn((GameObjectBundle::new("barrel", pos()), Barrel::Fire));
                }
            }
            ConveyorOutput::BarrelsFrequent => {
                if check(Duration::from_millis(3000), Duration::from_millis(1200), 3) {
                    commands.spawn((GameObjectBundle::new("barrel", pos()), Barrel::Fire));
                }
            }
            ConveyorOutput::RandomEnemies => {
                if check(Duration::from_millis((5000..12000).random()), default(), 1) {
                    commands.spawn((GameObjectBundle::new("barrel", pos()), Enemy::Spam));
                }
            }
            ConveyorOutput::Random => {
                if check(
                    Duration::from_millis((2000..6000).random()),
                    Duration::from_millis((4000..20000).random()),
                    (1..4).random(),
                ) {
                    match (0. ..1.).random() < 0.1 {
                        true => {
                            commands.spawn((GameObjectBundle::new("enemy", pos()), Enemy::Spam))
                        }
                        false => {
                            commands.spawn((GameObjectBundle::new("barrel", pos()), Barrel::Fire))
                        }
                    };
                }
            }
        }
    }
}

fn chute_destroy(
    colliding: Query<&CollidingEntities, With<ChuteDespawn>>,
    player: Query<Has<Dead>, With<Player>>,
    mut commands: Commands,
) {
    for colliding in colliding.iter() {
        for entity in colliding.iter() {
            if let Ok(dead) = player.get(entity) {
                if !dead {
                    commands.try_insert(entity, Dead);
                }
            } else {
                commands.try_despawn_recursive(entity);
            }
        }
    }
}
