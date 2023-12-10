use super::barrels::Barrel;
use super::barrels::Explosion;
use super::barrels::OnFire;
use crate::gameplay::balance::*;
use crate::gameplay::master::level::spawn::GameObjectBundle;
use crate::gameplay::mechanics::damage::ApplyDamage;
use crate::gameplay::mechanics::damage::DamageType;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::mechanics::damage::Health;
use crate::gameplay::mechanics::damage::Projectile;
use crate::gameplay::mechanics::damage::ProjectileImpact;
use crate::gameplay::mechanics::overload::Overload;
use crate::gameplay::mechanics::overload::OverloadSource;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::*;
use crate::gameplay::utils::Lifetime;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::math_algorithms::dir_vec2;
use crate::utils::misc_utils::ExtendedTime;
use crate::utils::random::RandomRange;
use crate::utils::random::RandomVec;
use bevy::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component, Clone, Copy)]
pub enum Particle {
    ProjectileImpact,
    FireImpact,
    ColdFire,
    Shockwave,
    OverloadedSparks,
}

pub struct ParticleDescriptor {
    size: f32,
    pub graphical_size: f32,
    distance: f32,
    graphical_count: usize,
    overload_power: f32,
    overload_lifetime: Option<Duration>,
    pub lifetime: Duration,
    pub z_offset: f32,
}

impl Particle {
    pub fn descriptor(self) -> ParticleDescriptor {
        match self {
            Particle::ProjectileImpact => ParticleDescriptor {
                size: 0.1,
                graphical_size: 0.7,
                distance: 1.,
                graphical_count: 8,
                overload_power: OVERLOAD_PROJECTILE_IMPACT,
                overload_lifetime: Some(OVERLOAD_DURATION_PARTICLE),
                lifetime: Duration::from_millis(1200),
                z_offset: 1.,
            },

            Particle::FireImpact => ParticleDescriptor {
                size: 0.01,
                graphical_size: 0.5,
                distance: 0.5,
                graphical_count: 4,
                overload_power: OVERLOAD_FIRE_IMPACT,
                overload_lifetime: Some(OVERLOAD_DURATION_PARTICLE),
                lifetime: Duration::from_millis(600),
                z_offset: 1.2,
            },

            Particle::ColdFire => ParticleDescriptor {
                size: 0.1,
                graphical_size: 1.,
                distance: 1.,
                graphical_count: 8,
                overload_power: 0.,
                overload_lifetime: None,
                lifetime: Duration::from_millis(800),
                z_offset: 1.2,
            },

            Particle::Shockwave => ParticleDescriptor {
                size: 0.3,
                graphical_size: 2.,
                distance: 1.,
                graphical_count: 1,
                overload_power: 0.,
                overload_lifetime: None,
                lifetime: Duration::from_millis(300),
                z_offset: 0.6,
            },

            Particle::OverloadedSparks => ParticleDescriptor {
                size: 0.1,
                graphical_size: 0.1,
                distance: 1.,
                graphical_count: 4,
                overload_power: 0.,
                overload_lifetime: None,
                lifetime: Duration::from_millis(300),
                z_offset: 2.,
            },
        }
    }

    fn graphical_bundle(self, pos: Vec2, end_delta: Vec2) -> impl Bundle {
        let descr = self.descriptor();
        (
            GameObjectBundle::new("projectile", Transform::from_translation(pos.extend(0.))),
            Lifetime(descr.lifetime),
            //
            RigidBody::Dynamic,
            Collider::ball(descr.size),
            Restitution {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Max,
            },
            Velocity::linear(end_delta / descr.lifetime.as_secs_f32()),
            PhysicsType::WallOnly.groups(),
            //
            self,
        )
    }

    fn overload_bundle(self, pos: Vec2) -> impl Bundle {
        let descr = self.descriptor();
        let lifetime = descr.overload_lifetime.unwrap_or(descr.lifetime);
        (
            GameObjectBundle::new(
                "projectile overload",
                Transform::from_translation(pos.extend(0.)),
            ),
            Lifetime(lifetime),
            //
            Collider::ball(descr.size),
            PhysicsType::Overload.groups(),
            OverloadSource {
                power: descr.overload_power,
            },
        )
    }
}

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (particle_events, on_fire, on_explosion, on_overload).after(MechanicSet::PostReaction),
        );
    }
}

fn spawn_particle(commands: &mut Commands, pos: Vec2, ty: Particle) {
    let descr = ty.descriptor();

    for _ in 0..descr.graphical_count {
        let dir = Vec2::random_dir() * (descr.distance * 0.5..descr.distance * 1.5).random();
        commands.spawn(ty.graphical_bundle(pos, dir));
    }

    if descr.overload_power > 0. {
        commands.spawn(ty.overload_bundle(pos));
    }
}

fn particle_events(mut projectile_impact: EventReader<ProjectileImpact>, mut commands: Commands) {
    for ProjectileImpact { pos, projectile } in projectile_impact.read().copied() {
        let mut spawn = |ty| spawn_particle(&mut commands, pos, ty);

        match projectile.ty {
            DamageType::Player => spawn(Particle::ProjectileImpact),
            DamageType::Barrels => {
                spawn(Particle::FireImpact);
                spawn(Particle::ColdFire);
            }
        };
    }
}

fn on_fire(
    barrels: Query<(&GlobalTransform, &Velocity, &OnFire)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let period = Duration::from_millis(500);

    for (pos, velocity, fire) in barrels.iter() {
        if time.is_tick(period, fire.started_at) {
            for _ in 0..2 {
                let pos = pos.translation().truncate();
                let delta = velocity.linvel + Vec2::random_dir() * (0.3..1.).random();

                commands.spawn((Particle::FireImpact.graphical_bundle(pos, delta),));
                commands.spawn((Particle::ColdFire.graphical_bundle(pos, delta * 0.7),));
            }
        }
    }
}

fn on_explosion(
    mut explosions: EventReader<Explosion>,
    mut commands: Commands,
    mut damage: EventWriter<ApplyDamage>,
    physics: Res<RapierContext>,
    victims: Query<&GlobalTransform, With<Health>>,
) {
    let shockwave_distance = 3.;
    let shockwave_shape = Collider::ball(shockwave_distance - 0.4);

    for Explosion { at, ty } in explosions.read() {
        let pos = at.truncate();
        let dir = |distance: f32| Vec2::random_dir() * (distance * 0.5..distance * 1.5).random();

        let mut spawn = |ty: Particle| {
            let descr = ty.descriptor();
            for _ in 0..descr.graphical_count {
                commands.spawn(ty.graphical_bundle(pos, dir(descr.distance)));
            }
        };

        match ty {
            Barrel::Fire => {
                // graphics
                spawn(Particle::FireImpact);
                spawn(Particle::ColdFire);

                // shockwave
                let count = 16;
                for index in 0..count {
                    let ad = TAU / count as f32 * TAU;
                    let angle = ad * index as f32 + (-ad..ad).random() * 0.1;

                    let ty = Particle::Shockwave;
                    commands.spawn(ty.graphical_bundle(pos, dir_vec2(angle) * shockwave_distance));
                }

                // fireballs
                let count = 3;
                for _ in 0..count {
                    let direction = Vec2::random_dir();
                    commands.spawn(
                        Projectile {
                            damage: 1,
                            speed: 6.,
                            radius: 0.5,
                            ty: DamageType::Barrels,
                        }
                        .bundle(pos + direction * 0.5, direction),
                    );
                }

                // overload
                commands.spawn((
                    GameObjectBundle::new("explosion", Transform::from_translation(pos.extend(0.))),
                    Lifetime(OVERLOAD_DURATION_EXPLOSION),
                    //
                    Collider::ball(1.),
                    PhysicsType::Overload.groups(),
                    OverloadSource {
                        power: OVERLOAD_EXPLOSION,
                    },
                ));

                // apply damage
                physics.intersections_with_shape(
                    pos,
                    0.,
                    &shockwave_shape,
                    PhysicsType::Object.filter(),
                    |victim| {
                        if let Ok(transform) = victims.get(victim) {
                            damage.send(ApplyDamage {
                                victim,
                                amount: 3,
                                ty: DamageType::Barrels,
                            });
                            damage.send(ApplyDamage {
                                victim,
                                amount: 2,
                                ty: DamageType::Player,
                            });
                            commands.try_insert(
                                victim,
                                ExternalImpulse {
                                    impulse: {
                                        let target = transform.translation().truncate();
                                        let delta = target - pos;
                                        let delta = delta.normalize_or_zero();
                                        delta * 500.
                                    },
                                    ..default()
                                },
                            );
                        }
                        true
                    },
                );
            }
        }
    }
}

#[derive(Component)]
struct OverloadedSince(Duration);

fn on_overload(
    new: Query<(Entity, &GlobalTransform), (With<Overload>, Added<Dead>)>,
    overloaded: Query<(Entity, &OverloadedSince, &GlobalTransform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let period_1 = Duration::from_millis(30);
    let duration_1 = Duration::from_millis(1500);
    let period_2 = Duration::from_millis(300);
    let duration_2 = Duration::from_millis(8000);

    for (entity, pos) in new.iter() {
        commands.try_insert(entity, OverloadedSince(time.elapsed()));

        let pos = pos.translation().truncate();
        commands.spawn((
            GameObjectBundle::new("overloaded", Transform::from_translation(pos.extend(0.))),
            Lifetime(OVERLOAD_DURATION_OVERLOADED),
            //
            Collider::ball(1.),
            PhysicsType::Overload.groups(),
            OverloadSource {
                power: OVERLOAD_OVERLOADED,
            },
        ));
    }

    for (entity, since, pos) in overloaded.iter() {
        let passed = time.elapsed().saturating_sub(since.0);

        let period = if passed < duration_1 {
            period_1
        } else if passed < duration_2 {
            period_2
        } else {
            commands.try_remove::<OverloadedSince>(entity);
            continue;
        };

        if time.is_tick(period, since.0) {
            let pos = pos.translation().truncate();
            spawn_particle(&mut commands, pos, Particle::OverloadedSparks);
        }
    }
}
