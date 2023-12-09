use crate::gameplay::balance::OVERLOAD_PROJECTILE_IMPACT;
use crate::gameplay::master::level::spawn::GameObjectBundle;
use crate::gameplay::mechanics::damage::ProjectileImpact;
use crate::gameplay::mechanics::overload::OverloadSource;
use crate::gameplay::mechanics::MechanicSet;
use crate::gameplay::physics::*;
use crate::gameplay::utils::InterpolateTransformOnce;
use crate::gameplay::utils::Lifetime;
use crate::utils::random::RandomRange;
use crate::utils::random::RandomVec;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Clone, Copy)]
pub enum Particle {
    ProjectileImpact,
}

struct ParticleDescriptor {
    size: f32,
    distance: f32,
    graphical_count: usize,
    overload_power: f32,
    lifetime: Duration,
}

impl Particle {
    fn descriptor(self) -> ParticleDescriptor {
        match self {
            Self::ProjectileImpact => ParticleDescriptor {
                size: 0.1,
                distance: 1.,
                graphical_count: 8,
                overload_power: OVERLOAD_PROJECTILE_IMPACT,
                lifetime: Duration::from_millis(1200),
            },
        }
    }

    fn graphical_bundle(self, pos: Vec2, end_delta: Vec2) -> impl Bundle {
        let descr = self.descriptor();
        (
            GameObjectBundle::new("projectile", Transform::from_translation(pos.extend(0.))),
            Lifetime(descr.lifetime),
            // InterpolateTransformOnce::new(lifetime)
            //     .pos((pos + end_delta).extend(0.))
            //     .scale(Vec3::ZERO), // to use without physics
            InterpolateTransformOnce::new(descr.lifetime).scale(Vec3::ZERO),
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
        let ty = match projectile.damage {
            _ => Particle::ProjectileImpact,
        };
        let descr = ty.descriptor();

        for _ in 0..descr.graphical_count {
            let dir = Vec2::random_dir() * (descr.distance * 0.5..descr.distance * 1.5).random();
            commands.spawn(ty.graphical_bundle(pos, dir));
        }

        commands.spawn(ty.overload_bundle(pos));
    }
}
