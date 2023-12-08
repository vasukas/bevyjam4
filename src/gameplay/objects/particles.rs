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

impl Particle {
    pub fn size(self) -> f32 {
        match self {
            Self::ProjectileImpact => 0.1,
        }
    }

    fn overload_power(self) -> f32 {
        match self {
            Self::ProjectileImpact => 0.25,
        }
    }

    fn bundle(self, pos: Vec2, end_delta: Vec2) -> impl Bundle {
        let lifetime = match self {
            Particle::ProjectileImpact => Duration::from_millis(1200),
        };
        (
            GameObjectBundle::new("projectile", Transform::from_translation(pos.extend(0.))),
            Lifetime(lifetime),
            InterpolateTransformOnce::new(
                Transform::from_translation((pos + end_delta).extend(0.)).with_scale(Vec3::ZERO),
                lifetime,
            ),
            //
            RigidBody::Dynamic,
            Collider::ball(self.size()),
            Restitution {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Max,
            },
            PhysicsType::WallOnly.groups(),
            //
            OverloadSource {
                power: self.overload_power(),
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
    for ProjectileImpact { pos, damage } in projectile_impact.read().copied() {
        let (count, ty, base_distance) = match damage {
            _ => (8, Particle::ProjectileImpact, 1.),
        };

        for _ in 0..count {
            let dir = Vec2::random_dir() * (base_distance * 0.5..base_distance * 1.5).random();
            commands.spawn(ty.bundle(pos, dir));
        }
    }
}
