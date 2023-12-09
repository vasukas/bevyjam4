use crate::app::scheduling::SpawnSet;
use crate::gameplay::balance::OVERLOAD_BURNING_BARREL;
use crate::gameplay::mechanics::damage::DamageType;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::mechanics::overload::OverloadSource;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::*;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

/// A barrel. Can be grabbed and set on fire.
#[derive(Component, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum Barrel {
    Fire,
}

/// Added when barrel is ignited
#[derive(Component)]
pub struct OnFire {
    pub started_at: Duration,
    explode_at: Duration,
}

/// Sent when barrel explodes
#[derive(Event)]
pub struct Explosion {
    pub at: Vec3,
    pub ty: Barrel,
}

///
pub struct BarrelsPlugin;

impl Plugin for BarrelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Explosion>()
            .add_systems(PostUpdate, spawn_barrels.in_set(SpawnSet::Roots))
            .add_systems(
                Update,
                (put_barrels_on_fire, explode_barrels).in_set(MechanicSet::PostReaction),
            );
    }
}

const BARREL_HEALTH: u32 = 4;
const ON_FIRE_DURATION: Duration = Duration::from_millis(1700);

fn spawn_barrels(new: Query<Entity, Added<Barrel>>, mut commands: Commands) {
    for entity in new.iter() {
        commands.try_insert(
            entity,
            (
                TypicalBody::new_ball(0.45).mass(50.),
                PhysicsType::Object.groups(),
                Damping {
                    linear_damping: 0.8,
                    angular_damping: 0.5,
                },
                Velocity::default(),
                //
                Health {
                    value: BARREL_HEALTH,
                    ty: DamageType::Barrels,
                },
            ),
        );
    }
}

fn put_barrels_on_fire(
    barrels: Query<(Entity, &Health, &Barrel), (Changed<Health>, Without<OnFire>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, health, barrel) in barrels.iter() {
        match barrel {
            Barrel::Fire => {
                if health.value != BARREL_HEALTH {
                    commands.try_insert(
                        entity,
                        (
                            OnFire {
                                started_at: time.elapsed(),
                                explode_at: time.elapsed() + ON_FIRE_DURATION,
                            },
                            OverloadSource {
                                power: OVERLOAD_BURNING_BARREL,
                            },
                        ),
                    );
                }
            }
        }
    }
}

fn explode_barrels(
    barrels_dead: Query<(Entity, &GlobalTransform, &Barrel), Added<Dead>>,
    barrels_on_fire: Query<(Entity, &GlobalTransform, &Barrel, &OnFire)>,
    mut commands: Commands,
    time: Res<Time>,
    mut explosions: EventWriter<Explosion>,
) {
    let barrels = barrels_dead.iter().chain(
        barrels_on_fire
            .iter()
            .filter_map(|v| (time.elapsed() >= v.3.explode_at).then_some((v.0, v.1, v.2))),
    );
    for (entity, pos, barrel) in barrels {
        commands.try_despawn_recursive(entity);
        explosions.send(Explosion {
            at: pos.translation(),
            ty: *barrel,
        })
    }
}
