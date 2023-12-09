use crate::gameplay::balance::*;
use crate::gameplay::master::level::spawn::GameObjectBundle;
use crate::gameplay::mechanics::damage::DamageType;
use crate::gameplay::mechanics::damage::ProjectileImpact;
use crate::gameplay::mechanics::overload::OverloadSource;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::*;
use crate::gameplay::utils::Lifetime;
use crate::utils::random::RandomRange;
use crate::utils::random::RandomVec;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Clone, Copy)]
pub enum Particle {
    ProjectileImpact,
    FireImpact,
    ColdFire,
}

pub struct ParticleDescriptor {
    size: f32,
    pub graphical_size: f32,
    distance: f32,
    graphical_count: usize,
    overload_power: f32,
    pub lifetime: Duration,
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
                lifetime: Duration::from_millis(1200),
            },
            Particle::FireImpact => ParticleDescriptor {
                size: 0.01,
                graphical_size: 0.5,
                distance: 0.5,
                graphical_count: 4,
                overload_power: OVERLOAD_FIRE_IMPACT,
                lifetime: Duration::from_millis(600),
            },
            Particle::ColdFire => ParticleDescriptor {
                size: 0.1,
                graphical_size: 1.,
                distance: 1.,
                graphical_count: 8,
                overload_power: 0.,
                lifetime: Duration::from_millis(800),
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
        (
            GameObjectBundle::new(
                "projectile overload",
                Transform::from_translation(pos.extend(0.)),
            ),
            Lifetime(descr.lifetime),
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
        app.add_systems(Update, particle_events.after(MechanicSet::Reaction));
    }
}

fn particle_events(mut projectile_impact: EventReader<ProjectileImpact>, mut commands: Commands) {
    for ProjectileImpact { pos, projectile } in projectile_impact.read().copied() {
        let mut spawn = |ty: Particle| {
            let descr = ty.descriptor();

            for _ in 0..descr.graphical_count {
                let dir =
                    Vec2::random_dir() * (descr.distance * 0.5..descr.distance * 1.5).random();
                commands.spawn(ty.graphical_bundle(pos, dir));
            }

            if descr.overload_power > 0. {
                commands.spawn(ty.overload_bundle(pos));
            }
        };

        match projectile.ty {
            DamageType::Player => spawn(Particle::ProjectileImpact),
            DamageType::Barrels => {
                spawn(Particle::FireImpact);
                spawn(Particle::ColdFire)
            }
        };
    }
}
